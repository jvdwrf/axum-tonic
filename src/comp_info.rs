use axum::{async_trait, Server};

use crate::proto::company_info_server::*;
use crate::proto::*;
use tonic::{Request, Response, Status};

pub fn service() -> CompanyInfoServer<CompanyInfoService> {
    

    CompanyInfoServer::new(CompanyInfoService {})
}

fn some<T>(str: impl Into<T>) -> Option<T> {
    Some(str.into())
}

pub struct CompanyInfoService {}

#[async_trait]
impl CompanyInfo for CompanyInfoService {
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