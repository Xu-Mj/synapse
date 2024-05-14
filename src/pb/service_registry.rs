// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceInstance {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub port: i32,
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    #[prost(enumeration = "ServiceType", tag = "6")]
    pub r#type: i32,
    #[prost(map = "string, string", tag = "7")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub health_check: ::core::option::Option<HealthCheck>,
    #[prost(enumeration = "ServiceStatus", tag = "10")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub interval: i32,
    #[prost(int32, tag = "3")]
    pub timeout: i32,
    #[prost(int32, tag = "4")]
    pub retries: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
/// 操作状态定义
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationStatus {
    /// 操作是否成功
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// 相关信息或错误消息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 服务实例标识定义，我们需要根据服务名称找到所在的服务池子
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceInstanceIdentifier {
    /// 服务实例唯一标识
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 服务名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// 查询请求定义
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// 可以按服务名称查询
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 查询响应定义
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    /// 查询结果，返回服务实例列表
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<ServiceInstance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub port: i32,
    #[prost(enumeration = "ServiceStatus", tag = "5")]
    pub active: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceType {
    Http = 0,
    Https = 1,
    Grpc = 2,
}
impl ServiceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceType::Http => "HTTP",
            ServiceType::Https => "HTTPS",
            ServiceType::Grpc => "GRPC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP" => Some(Self::Http),
            "HTTPS" => Some(Self::Https),
            "GRPC" => Some(Self::Grpc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceStatus {
    Up = 0,
    Down = 1,
}
impl ServiceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceStatus::Up => "UP",
            ServiceStatus::Down => "DOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UP" => Some(Self::Up),
            "DOWN" => Some(Self::Down),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod service_registry_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// gRPC服务接口定义，用于服务注册、注销和查询
    #[derive(Debug, Clone)]
    pub struct ServiceRegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceRegistryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServiceRegistryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ServiceRegistryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ServiceRegistryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// 注册服务实例
        pub async fn register_service(
            &mut self,
            request: impl tonic::IntoRequest<super::ServiceInstance>,
        ) -> std::result::Result<tonic::Response<super::OperationStatus>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_registry.ServiceRegistry/RegisterService",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "service_registry.ServiceRegistry",
                "RegisterService",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// 注销服务实例
        pub async fn unregister_service(
            &mut self,
            request: impl tonic::IntoRequest<super::ServiceInstanceIdentifier>,
        ) -> std::result::Result<tonic::Response<super::OperationStatus>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_registry.ServiceRegistry/UnregisterService",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "service_registry.ServiceRegistry",
                "UnregisterService",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// 查询服务实例
        pub async fn query_services(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service_registry.ServiceRegistry/QueryServices",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "service_registry.ServiceRegistry",
                "QueryServices",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Service>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/service_registry.ServiceRegistry/Subscribe");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "service_registry.ServiceRegistry",
                "Subscribe",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod service_registry_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ServiceRegistryServer.
    #[async_trait]
    pub trait ServiceRegistry: Send + Sync + 'static {
        /// 注册服务实例
        async fn register_service(
            &self,
            request: tonic::Request<super::ServiceInstance>,
        ) -> std::result::Result<tonic::Response<super::OperationStatus>, tonic::Status>;
        /// 注销服务实例
        async fn unregister_service(
            &self,
            request: tonic::Request<super::ServiceInstanceIdentifier>,
        ) -> std::result::Result<tonic::Response<super::OperationStatus>, tonic::Status>;
        /// 查询服务实例
        async fn query_services(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        /// Server streaming response type for the Subscribe method.
        type SubscribeStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Service, tonic::Status>,
            > + Send
            + 'static;
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscribeRequest>,
        ) -> std::result::Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
    }
    /// gRPC服务接口定义，用于服务注册、注销和查询
    #[derive(Debug)]
    pub struct ServiceRegistryServer<T: ServiceRegistry> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServiceRegistry> ServiceRegistryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServiceRegistryServer<T>
    where
        T: ServiceRegistry,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service_registry.ServiceRegistry/RegisterService" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterServiceSvc<T: ServiceRegistry>(pub Arc<T>);
                    impl<T: ServiceRegistry> tonic::server::UnaryService<super::ServiceInstance>
                        for RegisterServiceSvc<T>
                    {
                        type Response = super::OperationStatus;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServiceInstance>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ServiceRegistry>::register_service(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_registry.ServiceRegistry/UnregisterService" => {
                    #[allow(non_camel_case_types)]
                    struct UnregisterServiceSvc<T: ServiceRegistry>(pub Arc<T>);
                    impl<T: ServiceRegistry>
                        tonic::server::UnaryService<super::ServiceInstanceIdentifier>
                        for UnregisterServiceSvc<T>
                    {
                        type Response = super::OperationStatus;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServiceInstanceIdentifier>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ServiceRegistry>::unregister_service(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnregisterServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_registry.ServiceRegistry/QueryServices" => {
                    #[allow(non_camel_case_types)]
                    struct QueryServicesSvc<T: ServiceRegistry>(pub Arc<T>);
                    impl<T: ServiceRegistry> tonic::server::UnaryService<super::QueryRequest> for QueryServicesSvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ServiceRegistry>::query_services(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryServicesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service_registry.ServiceRegistry/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: ServiceRegistry>(pub Arc<T>);
                    impl<T: ServiceRegistry>
                        tonic::server::ServerStreamingService<super::SubscribeRequest>
                        for SubscribeSvc<T>
                    {
                        type Response = super::Service;
                        type ResponseStream = T::SubscribeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ServiceRegistry>::subscribe(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ServiceRegistry> Clone for ServiceRegistryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ServiceRegistry> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServiceRegistry> tonic::server::NamedService for ServiceRegistryServer<T> {
        const NAME: &'static str = "service_registry.ServiceRegistry";
    }
}
