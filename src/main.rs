use std::convert::Infallible;

use axum::{response::IntoResponse, routing::get, Router};
use futures::{Future, FutureExt};
use hyper::{service::Service, Request};
use tonic::transport::NamedService;

pub mod comp_info;
mod proto;

#[tokio::main]
async fn main() {
    let info_server = comp_info::service();
    let axum_router = Router::new().route("/", get(|| async move {}));

    let res = Router::new()
        .nest("/", axum_router)
        .nest_tonic(info_server)
        .route("/test", get(|| async move {}));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(res.into_make_service())
        .await
        .unwrap();
}

/// This trait automatically nests the router at the correct path, taken from NamedService
pub trait AxumTonic: Sized {
    fn nest_tonic<S>(self, svc: S) -> Self
    where
        S: Service<
                hyper::Request<hyper::Body>,
                Error = Infallible,
                Response = hyper::Response<tonic::body::BoxBody>,
            >
            + Clone
            + Send
            + 'static
            + NamedService,
        S::Future: Send + 'static + Unpin;
}

impl AxumTonic for Router {
    fn nest_tonic<S>(self, svc: S) -> Self
    where
        S: Service<
                hyper::Request<hyper::Body>,
                Error = Infallible,
                Response = hyper::Response<tonic::body::BoxBody>,
            >
            + Clone
            + Send
            + 'static
            + NamedService,
        S::Future: Send + 'static + Unpin,
    {
        // Nest it at /S::NAME, and wrap the service in an AxumTonicService
        self.nest(&format!("/{}", S::NAME), AxumTonicService { svc })
    }
}

/// The service that converts a tonic service into an axum-compatible one.
#[derive(Clone, Debug)]
struct AxumTonicService<S> {
    svc: S,
}

/// The future that is returned by the AxumTonicService
struct AxumTonicServiceFut<F> {
    fut: F,
}

impl<F> Future for AxumTonicServiceFut<F>
where
    F: Future<Output = Result<hyper::Response<tonic::body::BoxBody>, Infallible>> + Unpin,
{
    type Output = Result<axum::response::Response, Infallible>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // we only have to map this, whenever an actual response is returned
        self.fut.poll_unpin(cx).map_ok(map_response)
    }
}

// This is the part where the actual mapping is done.
// Ive got something that compiles here, but this definetely would not be correct in a lot of cases.
// There must be a way to map the correct parts here...
fn map_response(response: hyper::Response<tonic::body::BoxBody>) -> axum::response::Response {
    let (mut parts1, tonic_body) = response.into_parts();
    let (parts2, axum_body) = tonic_body.into_response().into_parts();

    let axum::http::response::Parts {
        status: status1,
        version: version1,
        headers: headers1,
        extensions: extensions1,
        ..
    } = &parts1;

    let axum::http::response::Parts {
        status: status2,
        version: version2,
        headers: headers2,
        extensions: extensions2,
        ..
    } = &parts2;

    parts1.extensions.extend(parts2.extensions);
    parts1.headers.extend(parts2.headers);

    axum::response::Response::from_parts(parts1, axum_body)
}

impl<B, S> Service<Request<B>> for AxumTonicService<S>
where
    S: Service<Request<B>, Error = Infallible, Response = hyper::Response<tonic::body::BoxBody>>,
    S::Future: Unpin,
{
    type Response = axum::response::Response;

    type Error = Infallible;

    type Future = AxumTonicServiceFut<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        AxumTonicServiceFut {
            fut: self.svc.call(req),
        }
    }
}
