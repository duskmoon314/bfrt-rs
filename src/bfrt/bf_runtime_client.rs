use tonic::codegen::http::Uri;
use tonic::codegen::*;
#[derive(Debug, Clone)]
pub struct BfRuntimeClient<T> {
    inner: tonic::client::Grpc<T>,
}
impl BfRuntimeClient<tonic::transport::Channel> {
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
impl<T> BfRuntimeClient<T>
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
    ) -> BfRuntimeClient<InterceptedService<T, F>>
    where
        F: tonic::service::Interceptor,
        T::ResponseBody: Default,
        T: tonic::codegen::Service<
            http::Request<tonic::body::Body>,
            Response = http::Response<
                <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
            >,
        >,
        <T as tonic::codegen::Service<http::Request<tonic::body::Body>>>::Error:
            Into<StdError> + std::marker::Send + std::marker::Sync,
    {
        BfRuntimeClient::new(InterceptedService::new(inner, interceptor))
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
    /// Update one or more P4 entities on the target.
    pub async fn write(
        &mut self,
        request: impl tonic::IntoRequest<super::WriteRequest>,
    ) -> std::result::Result<tonic::Response<super::WriteResponse>, tonic::Status> {
        self.inner
            .ready()
            .await
            .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/bfrt_proto.BfRuntime/Write");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("bfrt_proto.BfRuntime", "Write"));
        self.inner.unary(req, path, codec).await
    }
    /// Read one or more P4 entities from the target.
    pub async fn read(
        &mut self,
        request: impl tonic::IntoRequest<super::ReadRequest>,
    ) -> std::result::Result<
        tonic::Response<tonic::codec::Streaming<super::ReadResponse>>,
        tonic::Status,
    > {
        self.inner
            .ready()
            .await
            .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/bfrt_proto.BfRuntime/Read");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("bfrt_proto.BfRuntime", "Read"));
        self.inner.server_streaming(req, path, codec).await
    }
    /// Sets the P4 fowarding-pipeline config.
    pub async fn set_forwarding_pipeline_config(
        &mut self,
        request: impl tonic::IntoRequest<super::SetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::SetForwardingPipelineConfigResponse>,
        tonic::Status,
    > {
        self.inner
            .ready()
            .await
            .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static(
            "/bfrt_proto.BfRuntime/SetForwardingPipelineConfig",
        );
        let mut req = request.into_request();
        req.extensions_mut().insert(GrpcMethod::new(
            "bfrt_proto.BfRuntime",
            "SetForwardingPipelineConfig",
        ));
        self.inner.unary(req, path, codec).await
    }
    /// Gets the current P4 fowarding-pipeline config.
    pub async fn get_forwarding_pipeline_config(
        &mut self,
        request: impl tonic::IntoRequest<super::GetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::GetForwardingPipelineConfigResponse>,
        tonic::Status,
    > {
        self.inner
            .ready()
            .await
            .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static(
            "/bfrt_proto.BfRuntime/GetForwardingPipelineConfig",
        );
        let mut req = request.into_request();
        req.extensions_mut().insert(GrpcMethod::new(
            "bfrt_proto.BfRuntime",
            "GetForwardingPipelineConfig",
        ));
        self.inner.unary(req, path, codec).await
    }
    /// Represents the bidirectional stream between the controller and the
    /// switch (initiated by the controller).
    pub async fn stream_channel(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = super::StreamMessageRequest>,
    ) -> std::result::Result<
        tonic::Response<tonic::codec::Streaming<super::StreamMessageResponse>>,
        tonic::Status,
    > {
        self.inner
            .ready()
            .await
            .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/bfrt_proto.BfRuntime/StreamChannel");
        let mut req = request.into_streaming_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("bfrt_proto.BfRuntime", "StreamChannel"));
        self.inner.streaming(req, path, codec).await
    }
}
