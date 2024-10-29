/// Matches can be performed on arbitrarily-large inputs; the protobuf type
/// 'bytes' is used to model arbitrarily-large values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exact {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ternary {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub mask: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lpm {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// in bits
    #[prost(int32, tag = "2")]
    pub prefix_len: i32,
}
/// A Range is logically a set that contains all values numerically between
/// 'low' and 'high' inclusively.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(bytes = "vec", tag = "1")]
    pub low: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub high: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Optional {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub is_valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum MatchType {
    #[prost(message, tag = "2")]
    Exact(Exact),
    #[prost(message, tag = "3")]
    Ternary(Ternary),
    #[prost(message, tag = "4")]
    Lpm(Lpm),
    #[prost(message, tag = "5")]
    Range(Range),
    #[prost(message, tag = "6")]
    Optional(Optional),
}
