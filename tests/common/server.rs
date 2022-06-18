use axum::{async_trait, Server};

use crate::common::company_info::test_server::TestServer;
use crate::common::company_info::*;
use crate::common::company_info::{company_info_server::*, test_server::Test};
use tonic::{Request, Response, Status};

pub fn service1() -> CompanyInfoServer<MyCompanyInfoService> {
    CompanyInfoServer::new(MyCompanyInfoService {})
}

pub fn service2() -> TestServer<MyHiService> {
    TestServer::new(MyHiService {})
}

fn some<T>(str: impl Into<T>) -> Option<T> {
    Some(str.into())
}

pub struct MyHiService {}

impl Test for MyHiService {
    fn hi<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<crate::common::company_info::HiRequest>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<
                        tonic::Response<crate::common::company_info::HiReply>,
                        tonic::Status,
                    >,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { Ok(Response::new(HiReply {})) })
    }
}

pub struct MyCompanyInfoService {}

#[async_trait]
impl CompanyInfo for MyCompanyInfoService {
    async fn get_contact_info(
        &self,
        request: Request<GetContactInfoRequest>,
    ) -> Result<Response<GetContactInfoReply>, Status> {
        let meta = request.metadata();
        let ext = request.extensions().get::<u32>();
        Ok(Response::new(GetContactInfoReply {
            email: some("contact@wilt.com"),
        }))
    }

    async fn get_to_s(
        &self,
        request: Request<GetToSRequest>,
    ) -> Result<Response<GetTosReply>, Status> {
        Ok(Response::new(GetTosReply {
            terms_of_service: some("todo"),
        }))
    }

    async fn get_about_info(
        &self,
        request: Request<GetAboutInfoRequest>,
    ) -> Result<Response<GetAboutInfoReply>, Status> {
        Ok(Response::new(GetAboutInfoReply {
            about_info: some("todo"),
        }))
    }
}
