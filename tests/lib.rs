pub mod common;

use std::{net::SocketAddr, sync::Mutex, time::Duration};

use axum::{
    Router,
    extract::Request,
    middleware::{Next, from_fn},
    response::Response,
    routing::get,
};
use axum_tonic::{GrpcStatus, NestTonic, RestGrpcService};
use common::{
    proto::{
        Test1Request, Test2Request, test1_client::Test1Client, test1_server::Test1Server,
        test2_client::Test2Client, test2_server::Test2Server,
    },
    server::{Test1Service, Test2Service},
};
use tokio::net::TcpListener;
use tonic::transport::Channel;

use crate::common::server::{Test1ServiceWithConnectInfo, Test2ServiceWithConnectInfo};

async fn do_nothing(req: Request, next: Next) -> Result<Response, GrpcStatus> {
    Ok(next.run(req).await)
}

async fn cancel_request(_req: Request, _next: Next) -> Result<Response, GrpcStatus> {
    Err(tonic::Status::cancelled("Canceled").into())
}

#[tokio::test]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = Box::leak(Box::new(format!("http://127.0.0.1:{port}")));

    tokio::task::spawn(async move {
        let grpc_router1 = Router::new()
            .nest_tonic(Test1Server::new(Test1Service {
                state: Mutex::new(10),
                str: String::new(),
            }))
            .layer(from_fn(do_nothing));

        let grpc_router2 = Router::new()
            .nest_tonic(Test2Server::new(Test2Service))
            .layer(from_fn(cancel_request));

        let grpc_router = grpc_router1.merge(grpc_router2);

        let rest_router = Router::new().merge(Router::new().route("/123", get(|| async move {})));

        let service = RestGrpcService::new(rest_router, grpc_router).into_make_service();

        axum::serve(listener, service).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let channel = Channel::from_static(address).connect().await.unwrap();

    let mut client1 = Test1Client::new(channel.clone());
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();

    let channel = Channel::from_static(address).connect().await.unwrap();

    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();

    let mut client2 = Test2Client::new(channel);
    assert_eq!(
        client2.test2(Test2Request {}).await.unwrap_err().code(),
        tonic::Code::Cancelled,
    );
}

#[tokio::test]
async fn main_connect_info() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = Box::leak(Box::new(format!("http://127.0.0.1:{port}")));

    tokio::task::spawn(async move {
        let grpc_router1 = Router::new()
            .nest_tonic(Test1Server::new(Test1ServiceWithConnectInfo {
                state: Mutex::new(10),
                str: String::new(),
            }))
            .layer(from_fn(do_nothing));

        let grpc_router2 = Router::new()
            .nest_tonic(Test2Server::new(Test2ServiceWithConnectInfo))
            .layer(from_fn(cancel_request));

        let grpc_router = grpc_router1.merge(grpc_router2);

        let rest_router = Router::new().merge(Router::new().route("/123", get(|| async move {})));

        let service = RestGrpcService::new(rest_router, grpc_router).into_make_service_with_connect_info::<SocketAddr>();

        axum::serve(listener, service).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let channel = Channel::from_static(address).connect().await.unwrap();

    let mut client1 = Test1Client::new(channel.clone());
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();

    let channel = Channel::from_static(address).connect().await.unwrap();

    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();
    client1.test1(Test1Request {}).await.unwrap();

    let mut client2 = Test2Client::new(channel);
    assert_eq!(
        client2.test2(Test2Request {}).await.unwrap_err().code(),
        tonic::Code::Cancelled,
    );
}
