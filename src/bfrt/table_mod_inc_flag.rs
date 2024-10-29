#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// Enum to add the given data incrementally to the
    /// exising table entry
    ModIncAdd = 0,
    /// Enum to delete the given data from the
    /// exising table entry
    ModIncDelete = 1,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::ModIncAdd => "MOD_INC_ADD",
            Self::ModIncDelete => "MOD_INC_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MOD_INC_ADD" => Some(Self::ModIncAdd),
            "MOD_INC_DELETE" => Some(Self::ModIncDelete),
            _ => None,
        }
    }
}
