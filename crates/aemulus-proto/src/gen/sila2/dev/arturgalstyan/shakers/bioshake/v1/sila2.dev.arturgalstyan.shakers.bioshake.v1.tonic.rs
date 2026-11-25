// @generated
/// Generated client implementations.
pub mod bioshake_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BioshakeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BioshakeClient<tonic::transport::Channel> {
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
    impl<T> BioshakeClient<T>
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
        ) -> BioshakeClient<InterceptedService<T, F>>
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
            BioshakeClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn start_shaking(
            &mut self,
            request: impl tonic::IntoRequest<super::StartShakingParameters>,
        ) -> std::result::Result<
            tonic::Response<super::StartShakingResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/StartShaking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake",
                        "StartShaking",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_shaking(
            &mut self,
            request: impl tonic::IntoRequest<super::StopShakingParameters>,
        ) -> std::result::Result<
            tonic::Response<super::StopShakingResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/StopShaking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake",
                        "StopShaking",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn lock_plate(
            &mut self,
            request: impl tonic::IntoRequest<super::LockPlateParameters>,
        ) -> std::result::Result<
            tonic::Response<super::LockPlateResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/LockPlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake",
                        "LockPlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_plate(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockPlateParameters>,
        ) -> std::result::Result<
            tonic::Response<super::UnlockPlateResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/UnlockPlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake",
                        "UnlockPlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeSpeedParameters>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribeSpeedResponses>>,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/Subscribe_Speed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake",
                        "Subscribe_Speed",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_plate_locked(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePlateLockedParameters>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::SubscribePlateLockedResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/Subscribe_PlateLocked",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake",
                        "Subscribe_PlateLocked",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod bioshake_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BioshakeServer.
    #[async_trait]
    pub trait Bioshake: std::marker::Send + std::marker::Sync + 'static {
        async fn start_shaking(
            &self,
            request: tonic::Request<super::StartShakingParameters>,
        ) -> std::result::Result<
            tonic::Response<super::StartShakingResponses>,
            tonic::Status,
        >;
        async fn stop_shaking(
            &self,
            request: tonic::Request<super::StopShakingParameters>,
        ) -> std::result::Result<
            tonic::Response<super::StopShakingResponses>,
            tonic::Status,
        >;
        async fn lock_plate(
            &self,
            request: tonic::Request<super::LockPlateParameters>,
        ) -> std::result::Result<
            tonic::Response<super::LockPlateResponses>,
            tonic::Status,
        >;
        async fn unlock_plate(
            &self,
            request: tonic::Request<super::UnlockPlateParameters>,
        ) -> std::result::Result<
            tonic::Response<super::UnlockPlateResponses>,
            tonic::Status,
        >;
        /// Server streaming response type for the Subscribe_Speed method.
        type Subscribe_SpeedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SubscribeSpeedResponses, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn subscribe_speed(
            &self,
            request: tonic::Request<super::SubscribeSpeedParameters>,
        ) -> std::result::Result<
            tonic::Response<Self::Subscribe_SpeedStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the Subscribe_PlateLocked method.
        type Subscribe_PlateLockedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::SubscribePlateLockedResponses,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn subscribe_plate_locked(
            &self,
            request: tonic::Request<super::SubscribePlateLockedParameters>,
        ) -> std::result::Result<
            tonic::Response<Self::Subscribe_PlateLockedStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BioshakeServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> BioshakeServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BioshakeServer<T>
    where
        T: Bioshake,
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/StartShaking" => {
                    #[allow(non_camel_case_types)]
                    struct StartShakingSvc<T: Bioshake>(pub Arc<T>);
                    impl<
                        T: Bioshake,
                    > tonic::server::UnaryService<super::StartShakingParameters>
                    for StartShakingSvc<T> {
                        type Response = super::StartShakingResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartShakingParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bioshake>::start_shaking(&inner, request).await
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
                        let method = StartShakingSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/StopShaking" => {
                    #[allow(non_camel_case_types)]
                    struct StopShakingSvc<T: Bioshake>(pub Arc<T>);
                    impl<
                        T: Bioshake,
                    > tonic::server::UnaryService<super::StopShakingParameters>
                    for StopShakingSvc<T> {
                        type Response = super::StopShakingResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopShakingParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bioshake>::stop_shaking(&inner, request).await
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
                        let method = StopShakingSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/LockPlate" => {
                    #[allow(non_camel_case_types)]
                    struct LockPlateSvc<T: Bioshake>(pub Arc<T>);
                    impl<
                        T: Bioshake,
                    > tonic::server::UnaryService<super::LockPlateParameters>
                    for LockPlateSvc<T> {
                        type Response = super::LockPlateResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LockPlateParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bioshake>::lock_plate(&inner, request).await
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
                        let method = LockPlateSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/UnlockPlate" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockPlateSvc<T: Bioshake>(pub Arc<T>);
                    impl<
                        T: Bioshake,
                    > tonic::server::UnaryService<super::UnlockPlateParameters>
                    for UnlockPlateSvc<T> {
                        type Response = super::UnlockPlateResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnlockPlateParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bioshake>::unlock_plate(&inner, request).await
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
                        let method = UnlockPlateSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/Subscribe_Speed" => {
                    #[allow(non_camel_case_types)]
                    struct Subscribe_SpeedSvc<T: Bioshake>(pub Arc<T>);
                    impl<
                        T: Bioshake,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeSpeedParameters,
                    > for Subscribe_SpeedSvc<T> {
                        type Response = super::SubscribeSpeedResponses;
                        type ResponseStream = T::Subscribe_SpeedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeSpeedParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bioshake>::subscribe_speed(&inner, request).await
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
                        let method = Subscribe_SpeedSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake/Subscribe_PlateLocked" => {
                    #[allow(non_camel_case_types)]
                    struct Subscribe_PlateLockedSvc<T: Bioshake>(pub Arc<T>);
                    impl<
                        T: Bioshake,
                    > tonic::server::ServerStreamingService<
                        super::SubscribePlateLockedParameters,
                    > for Subscribe_PlateLockedSvc<T> {
                        type Response = super::SubscribePlateLockedResponses;
                        type ResponseStream = T::Subscribe_PlateLockedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribePlateLockedParameters,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bioshake>::subscribe_plate_locked(&inner, request)
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
                        let method = Subscribe_PlateLockedSvc(inner);
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
    impl<T> Clone for BioshakeServer<T> {
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
    pub const SERVICE_NAME: &str = "sila2.dev.arturgalstyan.shakers.bioshake.v1.Bioshake";
    impl<T> tonic::server::NamedService for BioshakeServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
