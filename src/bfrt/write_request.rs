#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Atomicity {
    /// Required. This is the default behavior. The batch is processed in a
    /// non-atomic manner from a dataplane point of view. Each operation within
    /// the batch must be attempted even if one or more encounter errors.
    /// Every dataplane packet is guaranteed to be processed according to
    /// table contents as they are between two individual operations of the
    /// batch, but there could be several packets processed that see each of
    /// these intermediate stages.
    ContinueOnError = 0,
    /// Optional. Operations within the batch are committed to dataplane until
    /// an error is encountered. At this point, the operations must be rolled
    /// back such that both software and dataplane state is consistent with the
    /// state before the batch was attempted. The resulting behavior is
    /// all-or-none, except the batch is not atomic from a data plane point of
    /// view. Every dataplane packet is guaranteed to be processed according to
    /// table contents as they are between two individual operations of the
    /// batch, but there could be several packets processed that see each of
    /// these intermediate stages.
    RollbackOnError = 1,
    /// Optional. Every dataplane packet is guaranteed to be processed according
    /// to table contents before the batch began, or after the batch completed
    /// and the operations were programmed to the hardware.
    /// The batch is therefore treated as a transaction.
    DataplaneAtomic = 2,
}
impl Atomicity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::ContinueOnError => "CONTINUE_ON_ERROR",
            Self::RollbackOnError => "ROLLBACK_ON_ERROR",
            Self::DataplaneAtomic => "DATAPLANE_ATOMIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTINUE_ON_ERROR" => Some(Self::ContinueOnError),
            "ROLLBACK_ON_ERROR" => Some(Self::RollbackOnError),
            "DATAPLANE_ATOMIC" => Some(Self::DataplaneAtomic),
            _ => None,
        }
    }
}
