/// P4 Pipeline Profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// profile name
    #[prost(string, tag = "1")]
    pub profile_name: ::prost::alloc::string::String,
    /// context json file contents
    #[prost(bytes = "vec", tag = "2")]
    pub context: ::prost::alloc::vec::Vec<u8>,
    /// Binary to execute
    #[prost(bytes = "vec", tag = "3")]
    pub binary: ::prost::alloc::vec::Vec<u8>,
    /// Array of pipe_scope.
    #[prost(uint32, repeated, tag = "4")]
    pub pipe_scope: ::prost::alloc::vec::Vec<u32>,
}
