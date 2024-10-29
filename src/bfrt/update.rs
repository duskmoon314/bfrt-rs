#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    Unspecified = 0,
    Insert = 1,
    Modify = 2,
    /// MODIFY_INC is used to add/delete the given data to/from the
    /// existing table entry incrementally.
    ModifyInc = 3,
    Delete = 4,
    InsertOrModify = 5,
    Reset = 6,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Insert => "INSERT",
            Self::Modify => "MODIFY",
            Self::ModifyInc => "MODIFY_INC",
            Self::Delete => "DELETE",
            Self::InsertOrModify => "INSERT_OR_MODIFY",
            Self::Reset => "RESET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "INSERT" => Some(Self::Insert),
            "MODIFY" => Some(Self::Modify),
            "MODIFY_INC" => Some(Self::ModifyInc),
            "DELETE" => Some(Self::Delete),
            "INSERT_OR_MODIFY" => Some(Self::InsertOrModify),
            "RESET" => Some(Self::Reset),
            _ => None,
        }
    }
}
