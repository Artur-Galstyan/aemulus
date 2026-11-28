// @generated
/// Generated client implementations.
pub mod shaker_controller_client {
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
    pub struct ShakerControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShakerControllerClient<tonic::transport::Channel> {
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
    impl<T> ShakerControllerClient<T>
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
        ) -> ShakerControllerClient<InterceptedService<T, F>>
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
            ShakerControllerClient::new(InterceptedService::new(inner, interceptor))
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/StartShaking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/StopShaking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/LockPlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/UnlockPlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
                        "UnlockPlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_min_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMinSpeedParameters>,
        ) -> std::result::Result<
            tonic::Response<super::GetMinSpeedResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Get_MinSpeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
                        "Get_MinSpeed",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_max_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMaxSpeedParameters>,
        ) -> std::result::Result<
            tonic::Response<super::GetMaxSpeedResponses>,
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Get_MaxSpeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
                        "Get_MaxSpeed",
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Subscribe_Speed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Subscribe_PlateLocked",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController",
                        "Subscribe_PlateLocked",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod shaker_controller_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ShakerControllerServer.
    #[async_trait]
    pub trait ShakerController: std::marker::Send + std::marker::Sync + 'static {
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
        async fn get_min_speed(
            &self,
            request: tonic::Request<super::GetMinSpeedParameters>,
        ) -> std::result::Result<
            tonic::Response<super::GetMinSpeedResponses>,
            tonic::Status,
        >;
        async fn get_max_speed(
            &self,
            request: tonic::Request<super::GetMaxSpeedParameters>,
        ) -> std::result::Result<
            tonic::Response<super::GetMaxSpeedResponses>,
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
    pub struct ShakerControllerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ShakerControllerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ShakerControllerServer<T>
    where
        T: ShakerController,
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/StartShaking" => {
                    #[allow(non_camel_case_types)]
                    struct StartShakingSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
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
                                <T as ShakerController>::start_shaking(&inner, request)
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/StopShaking" => {
                    #[allow(non_camel_case_types)]
                    struct StopShakingSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
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
                                <T as ShakerController>::stop_shaking(&inner, request).await
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/LockPlate" => {
                    #[allow(non_camel_case_types)]
                    struct LockPlateSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
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
                                <T as ShakerController>::lock_plate(&inner, request).await
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/UnlockPlate" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockPlateSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
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
                                <T as ShakerController>::unlock_plate(&inner, request).await
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Get_MinSpeed" => {
                    #[allow(non_camel_case_types)]
                    struct Get_MinSpeedSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
                    > tonic::server::UnaryService<super::GetMinSpeedParameters>
                    for Get_MinSpeedSvc<T> {
                        type Response = super::GetMinSpeedResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMinSpeedParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ShakerController>::get_min_speed(&inner, request)
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
                        let method = Get_MinSpeedSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Get_MaxSpeed" => {
                    #[allow(non_camel_case_types)]
                    struct Get_MaxSpeedSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
                    > tonic::server::UnaryService<super::GetMaxSpeedParameters>
                    for Get_MaxSpeedSvc<T> {
                        type Response = super::GetMaxSpeedResponses;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMaxSpeedParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ShakerController>::get_max_speed(&inner, request)
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
                        let method = Get_MaxSpeedSvc(inner);
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Subscribe_Speed" => {
                    #[allow(non_camel_case_types)]
                    struct Subscribe_SpeedSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
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
                                <T as ShakerController>::subscribe_speed(&inner, request)
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
                "/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController/Subscribe_PlateLocked" => {
                    #[allow(non_camel_case_types)]
                    struct Subscribe_PlateLockedSvc<T: ShakerController>(pub Arc<T>);
                    impl<
                        T: ShakerController,
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
                                <T as ShakerController>::subscribe_plate_locked(
                                        &inner,
                                        request,
                                    )
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
    impl<T> Clone for ShakerControllerServer<T> {
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
    pub const SERVICE_NAME: &str = "sila2.dev.arturgalstyan.shakers.shakercontroller.v1.ShakerController";
    impl<T> tonic::server::NamedService for ShakerControllerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
