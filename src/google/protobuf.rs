#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
