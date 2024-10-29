#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Entity {
    #[prost(message, tag = "1")]
    TableEntry(super::TableEntry),
    #[prost(message, tag = "2")]
    TableUsage(super::TableUsage),
    #[prost(message, tag = "3")]
    TableAttribute(super::TableAttribute),
    #[prost(message, tag = "4")]
    TableOperation(super::TableOperation),
    #[prost(message, tag = "5")]
    ObjectId(super::ObjectId),
    #[prost(message, tag = "6")]
    Handle(super::HandleId),
}
