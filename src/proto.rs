#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactInfoReply {
    #[prost(string, optional, tag="1")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetToSRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTosReply {
    #[prost(string, optional, tag="1")]
    pub terms_of_service: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAboutInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAboutInfoReply {
    #[prost(string, optional, tag="1")]
    pub about_info: ::core::option::Option<::prost::alloc::string::String>,
}
/// Generated server implementations.
pub mod company_info_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CompanyInfoServer.
    #[async_trait]
    pub trait CompanyInfo: Send + Sync + 'static {
        /// Get the contact info
        async fn get_contact_info(
            &self,
            request: tonic::Request<crate::proto::GetContactInfoRequest>,
        ) -> Result<tonic::Response<crate::proto::GetContactInfoReply>, tonic::Status>;
        /// Get the terms of service
        async fn get_to_s(
            &self,
            request: tonic::Request<crate::proto::GetToSRequest>,
        ) -> Result<tonic::Response<crate::proto::GetTosReply>, tonic::Status>;
        /// Get the about information
        async fn get_about_info(
            &self,
            request: tonic::Request<crate::proto::GetAboutInfoRequest>,
        ) -> Result<tonic::Response<crate::proto::GetAboutInfoReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CompanyInfoServer<T: CompanyInfo> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CompanyInfo> CompanyInfoServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CompanyInfoServer<T>
    where
        T: CompanyInfo,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/company_info.CompanyInfo/GetContactInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetContactInfoSvc<T: CompanyInfo>(pub Arc<T>);
                    impl<
                        T: CompanyInfo,
                    > tonic::server::UnaryService<super::GetContactInfoRequest>
                    for GetContactInfoSvc<T> {
                        type Response = super::GetContactInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetContactInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_contact_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetContactInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/company_info.CompanyInfo/GetToS" => {
                    #[allow(non_camel_case_types)]
                    struct GetToSSvc<T: CompanyInfo>(pub Arc<T>);
                    impl<
                        T: CompanyInfo,
                    > tonic::server::UnaryService<super::GetToSRequest>
                    for GetToSSvc<T> {
                        type Response = super::GetTosReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetToSRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_to_s(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetToSSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/company_info.CompanyInfo/GetAboutInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetAboutInfoSvc<T: CompanyInfo>(pub Arc<T>);
                    impl<
                        T: CompanyInfo,
                    > tonic::server::UnaryService<super::GetAboutInfoRequest>
                    for GetAboutInfoSvc<T> {
                        type Response = super::GetAboutInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAboutInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_about_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAboutInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CompanyInfo> Clone for CompanyInfoServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CompanyInfo> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CompanyInfo> tonic::transport::NamedService for CompanyInfoServer<T> {
        const NAME: &'static str = "company_info.CompanyInfo";
    }
}
