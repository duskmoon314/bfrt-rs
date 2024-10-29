#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Update {
    #[prost(message, tag = "2")]
    Subscribe(super::Subscribe),
    #[prost(message, tag = "3")]
    DigestAck(super::DigestListAck),
}
