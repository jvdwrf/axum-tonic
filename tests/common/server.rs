use std::sync::Mutex;

use crate::common::proto::test1_server::*;
use crate::common::proto::test2_server::*;
use crate::common::proto::*;
use axum::async_trait;
use tonic::Response;

pub struct Test1Service {
    pub state: Mutex<u32>,
    pub str: String
}
#[async_trait]
impl Test1 for Test1Service {
    async fn test1(
        &self,
        _request: tonic::Request<super::proto::Test1Request>,
    ) -> Result<tonic::Response<super::proto::Test1Reply>, tonic::Status> {

        *self.state.lock().unwrap() += 5;

        println!("{}", self.state.lock().unwrap().clone());

        Ok(Response::new(Test1Reply {}))
    }
}

pub struct Test2Service;
#[async_trait]
impl Test2 for Test2Service {
    async fn test2(
        &self,
        _request: tonic::Request<super::proto::Test2Request>,
    ) -> Result<tonic::Response<super::proto::Test2Reply>, tonic::Status> {
        Ok(Response::new(Test2Reply {}))
    }
}
