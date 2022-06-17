use std::{convert::Infallible, str::FromStr, time::Duration};

use axum::{
    http::{
        self,
        uri::{self, PathAndQuery},
    },
    middleware::{from_fn, Next},
    response::Response,
    routing::get,
    Router,
};
use company_info::GetAboutInfoRequest;
use futures::{Future, FutureExt};
use hyper::{service::Service, Request, StatusCode, Uri};
use tonic::{
    client::Grpc,
    transport::{Channel, NamedService},
};
use tower::layer::layer_fn;
use tower_http::trace::TraceLayer;

use crate::multiplex::MultiplexService;

pub mod server;
// mod proto;
pub mod client;
pub mod company_info;
pub mod multiplex;

async fn auth<B>(req: Request<B>, next: Next<B>, print: &str) -> Result<Response, StatusCode> {
    fn token_is_valid(token: &str) -> bool {
        true
    }
    
    println!("{}", print);

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    // match auth_header {
    //     Some(auth_header) if token_is_valid(auth_header) => Ok(next.run(req).await),
    //     _ => Err(StatusCode::UNAUTHORIZED),
    // }

    Ok(next.run(req).await)
}



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let handle = tokio::task::spawn(async move {
        let grpc_router1 = router_from_tonic(server::service1()).layer(from_fn(|a, b| auth(a, b, "router1")));

        let grpc_router2 = router_from_tonic(server::service2()).layer(from_fn(|a, b| auth(a, b, "router2")));

        let grpc_router = grpc_router1.merge(grpc_router2);

        let rest_router = Router::new()
            .nest("/", Router::new().route("/123", get(|| async move {})))
            .route("/", get(|| async move {}));

        let server = tower::make::Shared::new(MultiplexService::new(rest_router, grpc_router));

        axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
            .serve(server)
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let channel = Channel::from_static("http://127.0.0.1:3000")
        .connect()
        .await
        .unwrap();

    let mut client = company_info::company_info_client::CompanyInfoClient::new(channel);

    let response = client.get_about_info(GetAboutInfoRequest {}).await.unwrap();
    let response = client.get_about_info(GetAboutInfoRequest {}).await.unwrap();
    let response = client.get_about_info(GetAboutInfoRequest {}).await.unwrap();

    // println!("response: {:?}", response);
    // com

    // handle.await.unwrap();
}

//------------------------------------------------------------------------------------------------
//  Nest
//------------------------------------------------------------------------------------------------

pub fn router_from_tonic<S>(svc: S) -> Router
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
    Router::new().route(
        &format!("/{}/*grpc_service", S::NAME),
        AxumTonicService { svc },
    )
}

/// This trait automatically nests the router at the correct path, taken from NamedService
pub trait NestTonic: Sized {
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

impl NestTonic for Router {
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
        self.route(
            &format!("/{}/*grpc_service", S::NAME),
            AxumTonicService { svc },
        )
    }
}

//------------------------------------------------------------------------------------------------
//  Service
//------------------------------------------------------------------------------------------------

/// The service that converts a tonic service into an axum-compatible one.
#[derive(Clone, Debug)]
struct AxumTonicService<S> {
    svc: S,
}

impl<B, S> Service<Request<B>> for AxumTonicService<S>
where
    S: Service<Request<B>, Error = Infallible, Response = hyper::Response<tonic::body::BoxBody>>
        + NamedService,
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

//------------------------------------------------------------------------------------------------
//  Future
//------------------------------------------------------------------------------------------------

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
        self.fut.poll_unpin(cx).map_ok(|response| {
            let (parts, tonic_body) = response.into_parts();
            axum::response::Response::from_parts(parts, axum::body::boxed(tonic_body))
        })
    }
}
