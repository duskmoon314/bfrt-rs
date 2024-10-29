#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Names {
    #[prost(message, tag = "2")]
    ActionName(super::ActionName),
    #[prost(message, tag = "3")]
    KeyFieldName(super::KeyFieldName),
    #[prost(message, tag = "4")]
    DataFieldName(super::DataFieldName),
}
