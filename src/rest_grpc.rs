use axum::{Router, http::header::CONTENT_TYPE};
use futures::ready;
use hyper::{Request, Response};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::{Service, make::Shared};

/// This service splits all incoming requests either to the rest-service, or to
/// the grpc-service based on the `content-type` header.
///
/// Only if the header `content-type = application/grpc` exists, will the requests be handled
/// by the grpc-service. All other requests go to the rest-service.
#[derive(Debug, Clone)]
pub struct RestGrpcService {
    rest_router: Router,
    rest_ready: bool,
    grpc_router: Router,
    grpc_ready: bool,
}

impl RestGrpcService {
    /// Create a new service, which splits requests between the rest- and grpc-router.
    pub fn new(rest_router: Router, grpc_router: Router) -> Self {
        Self {
            rest_router,
            rest_ready: false,
            grpc_router,
            grpc_ready: false,
        }
    }

    /// Create a make-service from this service. This make-service can be directly used
    /// in the `serve` method of an axum/hyper Server.
    ///
    /// If you would like to add shared middleware for both the rest-service and the grpc-service,
    /// the following approach is recommended:
    ///
    /// ```ignore
    /// use axum_tonic::RestGrpcService;
    /// use tokio::net::TcpListener;
    /// use tower::ServiceBuilder;
    ///
    /// let svc: RestGrpcService = my_service();
    ///
    /// let svc_with_layers = ServiceBuilder::new()
    ///     .buffer(5)
    ///     .layer(my_layer1())
    ///     .layer(my_layer2())
    ///     .service(svc);
    ///
    /// axum::serve(TcpListener::bind(&"127.0.0.1:3000"), svc_with_layers)
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn into_make_service(self) -> Shared<Self> {
        Shared::new(self)
    }
}

impl<ReqBody> Service<Request<ReqBody>> for RestGrpcService
where
    ReqBody: http_body::Body<Data = axum::body::Bytes> + Send + 'static,
    ReqBody::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = Response<axum::body::Body>;
    type Error = Infallible;
    type Future = <Router as Service<Request<ReqBody>>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // drive readiness for each inner service and record which is ready
        loop {
            match (self.rest_ready, self.grpc_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(
                        <axum::Router as tower::Service<Request<ReqBody>>>::poll_ready(
                            &mut self.rest_router,
                            cx
                        )
                    )?;
                    self.rest_ready = true;
                }
                (_, false) => {
                    ready!(
                        <axum::Router as tower::Service<Request<ReqBody>>>::poll_ready(
                            &mut self.rest_router,
                            cx
                        )
                    )?;
                    self.grpc_ready = true;
                }
            }
        }
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // require users to call `poll_ready` first, if they don't we're allowed to panic
        // as per the `tower::Service` contract
        assert!(
            self.grpc_ready,
            "grpc service not ready. Did you forget to call `poll_ready`?"
        );
        assert!(
            self.rest_ready,
            "rest service not ready. Did you forget to call `poll_ready`?"
        );

        // if we get a grpc request call the grpc service, otherwise call the rest service
        // when calling a service it becomes not-ready so we have drive readiness again
        if is_grpc_request(&req) {
            self.grpc_ready = false;
            self.grpc_router.call(req)
        } else {
            self.rest_ready = false;
            self.rest_router.call(req)
        }
    }
}

fn is_grpc_request<B>(req: &Request<B>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .map(|content_type| content_type.as_bytes())
        .filter(|content_type| content_type.starts_with(b"application/grpc"))
        .is_some()
}
