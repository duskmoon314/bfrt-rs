#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionName {
    #[prost(string, tag = "1")]
    pub action: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyFieldName {
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataFieldName {
    #[prost(string, tag = "1")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub field: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableObject {
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(oneof = "table_object::Names", tags = "2, 3, 4")]
    pub names: ::core::option::Option<table_object::Names>,
}
/// Nested message and enum types in `TableObject`.
pub mod table_object;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LearnObject {
    #[prost(string, tag = "1")]
    pub learn_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data_field_name: ::core::option::Option<DataFieldName>,
}
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Object {
    #[prost(message, tag = "1")]
    TableObject(TableObject),
    #[prost(message, tag = "2")]
    LearnObject(LearnObject),
}
