#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PredefinedMode {
    All = 0,
    Single = 1,
}
impl PredefinedMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::All => "ALL",
            Self::Single => "SINGLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "SINGLE" => Some(Self::Single),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
pub enum Scope {
    #[prost(enumeration = "PredefinedMode", tag = "1")]
    Predef(i32),
    #[prost(uint64, tag = "2")]
    UserDefined(u64),
}
