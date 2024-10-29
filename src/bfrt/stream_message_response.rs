#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Update {
    /// This message is only used to let the server know
    #[prost(message, tag = "1")]
    Subscribe(super::Subscribe),
    /// of the existence of client with this client_id
    ///
    /// Learn Digest
    #[prost(message, tag = "2")]
    Digest(super::DigestList),
    /// Idle timeout notification
    #[prost(message, tag = "3")]
    IdleTimeoutNotification(super::IdleTimeoutNotification),
    /// Port status change notification
    #[prost(message, tag = "4")]
    PortStatusChangeNotification(super::PortStatusChgNotification),
    /// Response for a SetForwardingPipelineConfigRequest is sent here
    #[prost(message, tag = "5")]
    SetForwardingPipelineConfigResponse(super::SetForwardingPipelineConfigResponse),
}
