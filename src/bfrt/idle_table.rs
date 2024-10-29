#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdleTableMode {
    IdleTablePollMode = 0,
    IdleTableNotifyMode = 1,
}
impl IdleTableMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::IdleTablePollMode => "IDLE_TABLE_POLL_MODE",
            Self::IdleTableNotifyMode => "IDLE_TABLE_NOTIFY_MODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDLE_TABLE_POLL_MODE" => Some(Self::IdleTablePollMode),
            "IDLE_TABLE_NOTIFY_MODE" => Some(Self::IdleTableNotifyMode),
            _ => None,
        }
    }
}
