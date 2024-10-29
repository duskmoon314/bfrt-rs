#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Value {
    #[prost(message, tag = "2")]
    Key(super::TableKey),
    #[prost(uint32, tag = "3")]
    HandleId(u32),
}
