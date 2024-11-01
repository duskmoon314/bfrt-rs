use tonic::codegen::*;
/// Generated trait containing gRPC methods that should be implemented for use with BfRuntimeServer.
#[async_trait]
pub trait BfRuntime: std::marker::Send + std::marker::Sync + 'static {
    /// Update one or more P4 entities on the target.
    async fn write(
        &self,
        request: tonic::Request<super::WriteRequest>,
    ) -> std::result::Result<tonic::Response<super::WriteResponse>, tonic::Status>;
    /// Server streaming response type for the Read method.
    type ReadStream: tonic::codegen::tokio_stream::Stream<
            Item = std::result::Result<super::ReadResponse, tonic::Status>,
        > + std::marker::Send
        + 'static;
    /// Read one or more P4 entities from the target.
    async fn read(
        &self,
        request: tonic::Request<super::ReadRequest>,
    ) -> std::result::Result<tonic::Response<Self::ReadStream>, tonic::Status>;
    /// Sets the P4 fowarding-pipeline config.
    async fn set_forwarding_pipeline_config(
        &self,
        request: tonic::Request<super::SetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::SetForwardingPipelineConfigResponse>,
        tonic::Status,
    >;
    /// Gets the current P4 fowarding-pipeline config.
    async fn get_forwarding_pipeline_config(
        &self,
        request: tonic::Request<super::GetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::GetForwardingPipelineConfigResponse>,
        tonic::Status,
    >;
    /// Server streaming response type for the StreamChannel method.
    type StreamChannelStream: tonic::codegen::tokio_stream::Stream<
            Item = std::result::Result<super::StreamMessageResponse, tonic::Status>,
        > + std::marker::Send
        + 'static;
    /// Represents the bidirectional stream between the controller and the
    /// switch (initiated by the controller).
    async fn stream_channel(
        &self,
        request: tonic::Request<tonic::Streaming<super::StreamMessageRequest>>,
    ) -> std::result::Result<tonic::Response<Self::StreamChannelStream>, tonic::Status>;
}
#[derive(Debug)]
pub struct BfRuntimeServer<T> {
    inner: Arc<T>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
}
impl<T> BfRuntimeServer<T> {
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
impl<T, B> tonic::codegen::Service<http::Request<B>> for BfRuntimeServer<T>
where
    T: BfRuntime,
    B: Body + std::marker::Send + 'static,
    B::Error: Into<StdError> + std::marker::Send + 'static,
{
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        match req.uri().path() {
            "/bfrt_proto.BfRuntime/Write" => {
                #[allow(non_camel_case_types)]
                struct WriteSvc<T: BfRuntime>(pub Arc<T>);
                impl<T: BfRuntime> tonic::server::UnaryService<super::WriteRequest> for WriteSvc<T> {
                    type Response = super::WriteResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::WriteRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move { <T as BfRuntime>::write(&inner, request).await };
                        Box::pin(fut)
                    }
                }
                let accept_compression_encodings = self.accept_compression_encodings;
                let send_compression_encodings = self.send_compression_encodings;
                let max_decoding_message_size = self.max_decoding_message_size;
                let max_encoding_message_size = self.max_encoding_message_size;
                let inner = self.inner.clone();
                let fut = async move {
                    let method = WriteSvc(inner);
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
            "/bfrt_proto.BfRuntime/Read" => {
                #[allow(non_camel_case_types)]
                struct ReadSvc<T: BfRuntime>(pub Arc<T>);
                impl<T: BfRuntime> tonic::server::ServerStreamingService<super::ReadRequest> for ReadSvc<T> {
                    type Response = super::ReadResponse;
                    type ResponseStream = T::ReadStream;
                    type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::ReadRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move { <T as BfRuntime>::read(&inner, request).await };
                        Box::pin(fut)
                    }
                }
                let accept_compression_encodings = self.accept_compression_encodings;
                let send_compression_encodings = self.send_compression_encodings;
                let max_decoding_message_size = self.max_decoding_message_size;
                let max_encoding_message_size = self.max_encoding_message_size;
                let inner = self.inner.clone();
                let fut = async move {
                    let method = ReadSvc(inner);
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
            "/bfrt_proto.BfRuntime/SetForwardingPipelineConfig" => {
                #[allow(non_camel_case_types)]
                struct SetForwardingPipelineConfigSvc<T: BfRuntime>(pub Arc<T>);
                impl<T: BfRuntime>
                    tonic::server::UnaryService<super::SetForwardingPipelineConfigRequest>
                    for SetForwardingPipelineConfigSvc<T>
                {
                    type Response = super::SetForwardingPipelineConfigResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::SetForwardingPipelineConfigRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move {
                            <T as BfRuntime>::set_forwarding_pipeline_config(&inner, request).await
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
                    let method = SetForwardingPipelineConfigSvc(inner);
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
            "/bfrt_proto.BfRuntime/GetForwardingPipelineConfig" => {
                #[allow(non_camel_case_types)]
                struct GetForwardingPipelineConfigSvc<T: BfRuntime>(pub Arc<T>);
                impl<T: BfRuntime>
                    tonic::server::UnaryService<super::GetForwardingPipelineConfigRequest>
                    for GetForwardingPipelineConfigSvc<T>
                {
                    type Response = super::GetForwardingPipelineConfigResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::GetForwardingPipelineConfigRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move {
                            <T as BfRuntime>::get_forwarding_pipeline_config(&inner, request).await
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
                    let method = GetForwardingPipelineConfigSvc(inner);
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
            "/bfrt_proto.BfRuntime/StreamChannel" => {
                #[allow(non_camel_case_types)]
                struct StreamChannelSvc<T: BfRuntime>(pub Arc<T>);
                impl<T: BfRuntime> tonic::server::StreamingService<super::StreamMessageRequest>
                    for StreamChannelSvc<T>
                {
                    type Response = super::StreamMessageResponse;
                    type ResponseStream = T::StreamChannelStream;
                    type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<tonic::Streaming<super::StreamMessageRequest>>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut =
                            async move { <T as BfRuntime>::stream_channel(&inner, request).await };
                        Box::pin(fut)
                    }
                }
                let accept_compression_encodings = self.accept_compression_encodings;
                let send_compression_encodings = self.send_compression_encodings;
                let max_decoding_message_size = self.max_decoding_message_size;
                let max_encoding_message_size = self.max_encoding_message_size;
                let inner = self.inner.clone();
                let fut = async move {
                    let method = StreamChannelSvc(inner);
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
                    let res = grpc.streaming(method, req).await;
                    Ok(res)
                };
                Box::pin(fut)
            }
            _ => Box::pin(async move {
                let mut response = http::Response::new(empty_body());
                let headers = response.headers_mut();
                headers.insert(
                    tonic::Status::GRPC_STATUS,
                    (tonic::Code::Unimplemented as i32).into(),
                );
                headers.insert(
                    http::header::CONTENT_TYPE,
                    tonic::metadata::GRPC_CONTENT_TYPE,
                );
                Ok(response)
            }),
        }
    }
}
impl<T> Clone for BfRuntimeServer<T> {
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
pub const SERVICE_NAME: &str = "bfrt_proto.BfRuntime";
impl<T> tonic::server::NamedService for BfRuntimeServer<T> {
    const NAME: &'static str = SERVICE_NAME;
}
