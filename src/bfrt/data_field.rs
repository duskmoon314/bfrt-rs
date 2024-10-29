#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntArray {
    #[prost(uint32, repeated, tag = "1")]
    pub val: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolArray {
    #[prost(bool, repeated, tag = "1")]
    pub val: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrArray {
    #[prost(string, repeated, tag = "1")]
    pub val: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerArray {
    #[prost(message, repeated, tag = "1")]
    pub container: ::prost::alloc::vec::Vec<container_array::Container>,
}
/// Nested message and enum types in `ContainerArray`.
pub mod container_array;
/// All data fields are dealt with using a byte stream except for float
/// values. Float values are used for data fields for LPF and WRED table
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Value {
    #[prost(bytes, tag = "2")]
    Stream(::prost::alloc::vec::Vec<u8>),
    #[prost(float, tag = "3")]
    FloatVal(f32),
    #[prost(string, tag = "4")]
    StrVal(::prost::alloc::string::String),
    #[prost(message, tag = "5")]
    IntArrVal(IntArray),
    #[prost(message, tag = "6")]
    BoolArrVal(BoolArray),
    #[prost(message, tag = "7")]
    ContainerArrVal(ContainerArray),
    #[prost(bool, tag = "8")]
    BoolVal(bool),
    #[prost(message, tag = "9")]
    StrArrVal(StrArray),
}
