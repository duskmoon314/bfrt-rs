#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
    /// Entry changed from idle to active
    EntryActive = 0,
    /// Entry changed from active to idle
    EntryIdle = 1,
}
impl NotificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::EntryActive => "ENTRY_ACTIVE",
            Self::EntryIdle => "ENTRY_IDLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENTRY_ACTIVE" => Some(Self::EntryActive),
            "ENTRY_IDLE" => Some(Self::EntryIdle),
            _ => None,
        }
    }
}
