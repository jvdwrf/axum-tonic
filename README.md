# axum_tonic

[![Crates.io](https://img.shields.io/crates/v/axum_tonic)](https://crates.io/crates/axum)
[![Documentation](https://docs.rs/axum_tonic/badge.svg)](https://docs.rs/axum)

A tiny crate to use Tonic with Axum.

This crate makes it simple to use different kinds of middleware with different tonic-services.

The recommended way to use this is to create two separate root-routers, one for grpc and one for rest. Then both can be combined together at the root, and turned into a make service.

See the docs of Axum or Tonic for more information about the respective frameworks.

## Example

```rust

/// A middleware that does nothing, but just passes on the request.
async fn do_nothing_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, GrpcStatus> {
    Ok(next.run(req).await)
}

/// A middleware that cancels the request with a grpc status-code
async fn cancel_request_middleware<B>(_req: Request<B>, _next: Next<B>) -> Result<Response, GrpcStatus> {
    Err(tonic::Status::cancelled("Canceled").into())
}

#[tokio::main]
async fn main() {

    // Spawn the Server
    tokio::task::spawn(async move {
        // The first grpc-service has middleware that accepts the request.
        let grpc_router1 = Router::new()
            .nest_tonic(Test1Server::new(Test1Service))
            .layer(from_fn(do_nothing_middleware));

        // The second grpc-service instead cancels the request
        let grpc_router2 = Router::new()
            .nest_tonic(Test2Server::new(Test2Service))
            .layer(from_fn(cancel_request_middleware));

        // Merge both routers into one.
        let grpc_router = grpc_router1.merge(grpc_router2);

        // This is the normal rest-router, to which all normal requests are routed
        let rest_router = Router::new()
            .nest("/", Router::new().route("/123", get(|| async move {})))
            .route("/", get(|| async move {}));

        // Combine both services into one
        let service = RestGrpcService::new(rest_router, grpc_router);

        // And serve at 127.0.0.1:8080
        axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
            .serve(service.into_make_service())
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect to the server with a grpc-client
    let channel = Channel::from_static("http://127.0.0.1:8080")
        .connect()
        .await
        .unwrap();

    let mut client1 = Test1Client::new(channel.clone());
    let mut client2 = Test2Client::new(channel);

    // The first request will succeed
    client1.test1(Test1Request {}).await.unwrap();

    // While the second one gives a grpc Status::Canceled code.
    assert_eq!(
        client2.test2(Test2Request {}).await.unwrap_err().code(),
        tonic::Code::Cancelled,
    );
}
```