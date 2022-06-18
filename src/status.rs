use std::ops::{Deref, DerefMut};
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct TonicStatusWrapper(pub tonic::Status);

impl From<tonic::Status> for TonicStatusWrapper {
    fn from(s: tonic::Status) -> Self {
        Self(s)
    }
}

impl Deref for TonicStatusWrapper {
    type Target = tonic::Status;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TonicStatusWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoResponse for TonicStatusWrapper {
    fn into_response(self) -> Response {
        let (parts, tonic_body) = self.0.to_http().into_parts();
        axum::response::Response::from_parts(parts, axum::body::boxed(tonic_body))
    }
}
