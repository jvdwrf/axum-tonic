pub mod common;

use std::time::Duration;

use axum::{
    middleware::{from_fn, Next},
    response::Response,
    routing::get,
    Router, Server,
};
use axum_tonic::{NestTonic, RestGrpcService, TonicStatusWrapper};
use common::{
    company_info,
    company_info::{GetAboutInfoRequest, HiRequest},
    server::{self, *},
};
use hyper::Request;
use tonic::transport::Channel;

async fn tonic_middleware<B>(
    req: Request<B>,
    next: Next<B>,
    print: &str,
) -> Result<Response, TonicStatusWrapper> {
    println!("{}", print);
    Ok(next.run(req).await)

    // match true {
    //     true => Ok(next.run(req).await),
    //     false => Err(tonic::Status::cancelled("This request has been canceled").into()),
    // }
}

async fn tonic_middleware2<B>(
    req: Request<B>,
    next: Next<B>,
    print: &str,
) -> Result<Response, TonicStatusWrapper> {
    println!("{}", print);

    Ok(next.run(req).await)
    // match true {
    //     true => Ok(next.run(req).await),
    //     false => Err(tonic::Status::cancelled("This request has been canceled").into()),
    // }
}

#[tokio::test]
async fn main() {
    tokio::task::spawn(async move {
        let grpc_router1 = Router::new()
            .nest_tonic(server::service1())
            .layer(from_fn(|a, b| tonic_middleware(a, b, "router1")));

        let grpc_router2 = Router::new()
            .nest_tonic(server::service2())
            .layer(from_fn(|a, b| tonic_middleware2(a, b, "router2")));

        let grpc_router = grpc_router1.merge(grpc_router2);

        let rest_router = Router::new()
            .nest("/", Router::new().route("/123", get(|| async move {})))
            .route("/", get(|| async move {}));

        let server = RestGrpcService::new(rest_router, grpc_router).into_make_service();

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

    let channel2 = channel.clone();

    let mut client1 = company_info::company_info_client::CompanyInfoClient::new(channel);
    let mut client2 = company_info::test_client::TestClient::new(channel2);

    let response = client1
        .get_about_info(GetAboutInfoRequest {})
        .await
        .unwrap();
    let response = client1
        .get_about_info(GetAboutInfoRequest {})
        .await
        .unwrap();
    let response = client1
        .get_about_info(GetAboutInfoRequest {})
        .await
        .unwrap();

    let response = client2.hi(HiRequest {}).await.unwrap();
}
