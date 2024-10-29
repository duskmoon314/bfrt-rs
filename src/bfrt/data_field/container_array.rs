#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    #[prost(message, repeated, tag = "1")]
    pub val: ::prost::alloc::vec::Vec<super::super::DataField>,
}
