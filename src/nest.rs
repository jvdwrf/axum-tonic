use std::convert::Infallible;

use axum::{response::IntoResponse, Router};
use futures::{Future, FutureExt};
use hyper::{service::Service, Request};
use tonic::transport::NamedService;

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

    // fn nest_tonic_with_interceptor<S1, S2>(self, svc: S1, map_fn: fn(S1) -> S2) -> Self
    // where
    //     S1: Service<
    //             hyper::Request<hyper::Body>,
    //             Error = Infallible,
    //             Response = hyper::Response<tonic::body::BoxBody>,
    //         >
    //         + Clone
    //         + Send
    //         + 'static
    //         + NamedService,
    //     S1::Future: Send + 'static,
    //     S2: Service<
    //             hyper::Request<hyper::Body>,
    //             Error = Infallible,
    //             Response = hyper::Response<tonic::body::BoxBody>,
    //         > + Clone
    //         + Send
    //         + 'static,
    //     S2::Future: Send + 'static + Unpin;
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

    // fn nest_tonic_with_interceptor<S1, S2>(self, svc: S1, map_fn: fn(S1) -> S2) -> Self
    // where
    //     S1: Service<
    //             hyper::Request<hyper::Body>,
    //             Error = Infallible,
    //             Response = hyper::Response<tonic::body::BoxBody>,
    //         >
    //         + Clone
    //         + Send
    //         + 'static
    //         + NamedService,
    //     S1::Future: Send + 'static,
    //     S2: Service<
    //             hyper::Request<hyper::Body>,
    //             Error = Infallible,
    //             Response = hyper::Response<tonic::body::BoxBody>,
    //         > + Clone
    //         + Send
    //         + 'static,
    //     S2::Future: Send + 'static + Unpin,
    // {
    //     self.route(
    //         &format!("/{}/*grpc_service", S1::NAME),
    //         AxumTonicService { svc: map_fn(svc) },
    //     )
    // }
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
    S: Service<Request<B>, Error = Infallible, Response = hyper::Response<tonic::body::BoxBody>>,
    // + NamedService,
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
        self.fut
            .poll_unpin(cx)
            .map_ok(|response| response.into_response())
    }
}
