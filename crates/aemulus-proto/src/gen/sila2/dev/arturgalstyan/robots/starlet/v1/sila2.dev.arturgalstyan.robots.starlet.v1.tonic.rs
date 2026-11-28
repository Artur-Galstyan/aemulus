// @generated
/// Generated client implementations.
pub mod starlet_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Controls a Hamilton Microlab STARlet liquid handler.
    #[derive(Debug, Clone)]
    pub struct StarletClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StarletClient<tonic::transport::Channel> {
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
    impl<T> StarletClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> StarletClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            StarletClient::new(InterceptedService::new(inner, interceptor))
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
        /// Moves the X arm to a target position.
        pub async fn move_x(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveXParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveX",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveX",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Monitor the state of MoveX
        pub async fn move_x_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveX_Info",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveX_Info",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Retrieve result of MoveX
        pub async fn move_x_result(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<super::MoveXResponses>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveX_Result",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveX_Result",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moves the gripper Y axis to a target position.
        pub async fn move_y(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveYParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveY",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveY",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Monitor the state of MoveY
        pub async fn move_y_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveY_Info",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveY_Info",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Retrieve result of MoveY
        pub async fn move_y_result(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<super::MoveYResponses>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveY_Result",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveY_Result",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moves the gripper Z axis to a target position.
        pub async fn move_z(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveZParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveZ",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveZ",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Monitor the state of MoveZ
        pub async fn move_z_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveZ_Info",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveZ_Info",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Retrieve result of MoveZ
        pub async fn move_z_result(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<super::MoveZResponses>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveZ_Result",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "MoveZ_Result",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_gripper_width(
            &mut self,
            request: impl tonic::IntoRequest<super::SetGripperWidthParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/SetGripperWidth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "SetGripperWidth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_gripper_width_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/SetGripperWidth_Info",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "SetGripperWidth_Info",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn set_gripper_width_result(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetGripperWidthResponses>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/SetGripperWidth_Result",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet",
                        "SetGripperWidth_Result",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod starlet_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StarletServer.
    #[async_trait]
    pub trait Starlet: std::marker::Send + std::marker::Sync + 'static {
        /// Moves the X arm to a target position.
        async fn move_x(
            &self,
            request: tonic::Request<super::MoveXParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        >;
        /// Server streaming response type for the MoveX_Info method.
        type MoveX_InfoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        /// Monitor the state of MoveX
        async fn move_x_info(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<Self::MoveX_InfoStream>, tonic::Status>;
        /// Retrieve result of MoveX
        async fn move_x_result(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<super::MoveXResponses>, tonic::Status>;
        /// Moves the gripper Y axis to a target position.
        async fn move_y(
            &self,
            request: tonic::Request<super::MoveYParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        >;
        /// Server streaming response type for the MoveY_Info method.
        type MoveY_InfoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        /// Monitor the state of MoveY
        async fn move_y_info(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<Self::MoveY_InfoStream>, tonic::Status>;
        /// Retrieve result of MoveY
        async fn move_y_result(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<super::MoveYResponses>, tonic::Status>;
        /// Moves the gripper Z axis to a target position.
        async fn move_z(
            &self,
            request: tonic::Request<super::MoveZParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        >;
        /// Server streaming response type for the MoveZ_Info method.
        type MoveZ_InfoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        /// Monitor the state of MoveZ
        async fn move_z_info(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<Self::MoveZ_InfoStream>, tonic::Status>;
        /// Retrieve result of MoveZ
        async fn move_z_result(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<tonic::Response<super::MoveZResponses>, tonic::Status>;
        async fn set_gripper_width(
            &self,
            request: tonic::Request<super::SetGripperWidthParameters>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::org::silastandard::CommandConfirmation,
            >,
            tonic::Status,
        >;
        /// Server streaming response type for the SetGripperWidth_Info method.
        type SetGripperWidth_InfoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::super::super::super::org::silastandard::ExecutionInfo,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn set_gripper_width_info(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::SetGripperWidth_InfoStream>,
            tonic::Status,
        >;
        async fn set_gripper_width_result(
            &self,
            request: tonic::Request<
                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetGripperWidthResponses>,
            tonic::Status,
        >;
    }
    /// Controls a Hamilton Microlab STARlet liquid handler.
    #[derive(Debug)]
    pub struct StarletServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> StarletServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StarletServer<T>
    where
        T: Starlet,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveX" => {
                    #[allow(non_camel_case_types)]
                    struct MoveXSvc<T: Starlet>(pub Arc<T>);
                    impl<T: Starlet> tonic::server::UnaryService<super::MoveXParameters>
                    for MoveXSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::CommandConfirmation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveXParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_x(&inner, request).await
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
                        let method = MoveXSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveX_Info" => {
                    #[allow(non_camel_case_types)]
                    struct MoveX_InfoSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::ServerStreamingService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for MoveX_InfoSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::ExecutionInfo;
                        type ResponseStream = T::MoveX_InfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_x_info(&inner, request).await
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
                        let method = MoveX_InfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveX_Result" => {
                    #[allow(non_camel_case_types)]
                    struct MoveX_ResultSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::UnaryService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for MoveX_ResultSvc<T> {
                        type Response = super::MoveXResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_x_result(&inner, request).await
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
                        let method = MoveX_ResultSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveY" => {
                    #[allow(non_camel_case_types)]
                    struct MoveYSvc<T: Starlet>(pub Arc<T>);
                    impl<T: Starlet> tonic::server::UnaryService<super::MoveYParameters>
                    for MoveYSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::CommandConfirmation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveYParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_y(&inner, request).await
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
                        let method = MoveYSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveY_Info" => {
                    #[allow(non_camel_case_types)]
                    struct MoveY_InfoSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::ServerStreamingService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for MoveY_InfoSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::ExecutionInfo;
                        type ResponseStream = T::MoveY_InfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_y_info(&inner, request).await
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
                        let method = MoveY_InfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveY_Result" => {
                    #[allow(non_camel_case_types)]
                    struct MoveY_ResultSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::UnaryService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for MoveY_ResultSvc<T> {
                        type Response = super::MoveYResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_y_result(&inner, request).await
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
                        let method = MoveY_ResultSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveZ" => {
                    #[allow(non_camel_case_types)]
                    struct MoveZSvc<T: Starlet>(pub Arc<T>);
                    impl<T: Starlet> tonic::server::UnaryService<super::MoveZParameters>
                    for MoveZSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::CommandConfirmation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveZParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_z(&inner, request).await
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
                        let method = MoveZSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveZ_Info" => {
                    #[allow(non_camel_case_types)]
                    struct MoveZ_InfoSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::ServerStreamingService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for MoveZ_InfoSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::ExecutionInfo;
                        type ResponseStream = T::MoveZ_InfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_z_info(&inner, request).await
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
                        let method = MoveZ_InfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/MoveZ_Result" => {
                    #[allow(non_camel_case_types)]
                    struct MoveZ_ResultSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::UnaryService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for MoveZ_ResultSvc<T> {
                        type Response = super::MoveZResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::move_z_result(&inner, request).await
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
                        let method = MoveZ_ResultSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/SetGripperWidth" => {
                    #[allow(non_camel_case_types)]
                    struct SetGripperWidthSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::UnaryService<super::SetGripperWidthParameters>
                    for SetGripperWidthSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::CommandConfirmation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetGripperWidthParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::set_gripper_width(&inner, request).await
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
                        let method = SetGripperWidthSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/SetGripperWidth_Info" => {
                    #[allow(non_camel_case_types)]
                    struct SetGripperWidth_InfoSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::ServerStreamingService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for SetGripperWidth_InfoSvc<T> {
                        type Response = super::super::super::super::super::super::org::silastandard::ExecutionInfo;
                        type ResponseStream = T::SetGripperWidth_InfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::set_gripper_width_info(&inner, request)
                                    .await
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
                        let method = SetGripperWidth_InfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sila2.dev.arturgalstyan.robots.starlet.v1.Starlet/SetGripperWidth_Result" => {
                    #[allow(non_camel_case_types)]
                    struct SetGripperWidth_ResultSvc<T: Starlet>(pub Arc<T>);
                    impl<
                        T: Starlet,
                    > tonic::server::UnaryService<
                        super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                    > for SetGripperWidth_ResultSvc<T> {
                        type Response = super::SetGripperWidthResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::super::org::silastandard::CommandExecutionUuid,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Starlet>::set_gripper_width_result(&inner, request)
                                    .await
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
                        let method = SetGripperWidth_ResultSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for StarletServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sila2.dev.arturgalstyan.robots.starlet.v1.Starlet";
    impl<T> tonic::server::NamedService for StarletServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
