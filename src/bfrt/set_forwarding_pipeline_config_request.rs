#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// BIND: Default Action. Only binds the client to the program
    Bind = 0,
    /// specified in the p4_name. One client can bind to only one
    /// program. One program can have only one client as of now. Even
    /// in case of multiple programs on a single device, BIND requires
    /// just one program’s config msg. If multiple repeated
    /// forwarding_pipeline_config msgs are sent as part of this
    /// request, then google.rpc.INVALID_ARGUMENT is sent. If a client
    /// doesn't BIND, then it can only access
    /// SetForwardingPipelineConfigRequest,
    /// GetForwardingPipelineConfigRequest and StreamMessageRequest
    /// RPCs. Read and Write RPCs are not allowed for non-bound clients
    ///
    /// VERIFY(Master): Verifies whether this config is valid or not.
    Verify = 1,
    /// Upon failure or incomplete config in the msg,
    /// google.rpc.Code::INVALID_ARGUMENT is sent.
    ///
    /// VERIFY_AND_WARM_INIT_BEGIN(Master):  Verifies the config and then
    VerifyAndWarmInitBegin = 2,
    /// begins warm_init with this config. This does not modify the
    /// forwarding state of the device. However, any subsequent Read /
    /// Write requests must refer to fields in the new config. Returns an
    /// INVALID_ARGUMENT error if the forwarding config is not provided or
    /// if the provided config cannot be realized.
    ///
    /// VERIFY_AND_WARM_INIT_BEGIN_AND_END(Master): Verifies, starts
    VerifyAndWarmInitBeginAndEnd = 3,
    /// warm_init and then initiates warm_init_end on the switch. The
    /// existing forwarding state is reset. Returns an INVALID_ARGUMENT
    /// error if the forwarding config is not provided of if the provided
    /// config cannot be realized.
    ///
    /// WARM_INIT_END(Master): Issues a warm_init_end. If
    WarmInitEnd = 4,
    /// forwarding_pipeline_config contains anything, or if no
    /// WARM_INIT_BEGIN was previously called on the device
    /// with a valid config, then
    /// google.rpc.Code::INVALID_ARGUMENT is sent. The
    /// forwarding state in the target is updated by replaying
    /// the write requests to the target device since the last
    /// config was saved by the client.
    ///
    /// RECONCILE_AND_WARM_INIT_END(Master): Try and reconcile with the
    ReconcileAndWarmInitEnd = 5,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Bind => "BIND",
            Self::Verify => "VERIFY",
            Self::VerifyAndWarmInitBegin => "VERIFY_AND_WARM_INIT_BEGIN",
            Self::VerifyAndWarmInitBeginAndEnd => "VERIFY_AND_WARM_INIT_BEGIN_AND_END",
            Self::WarmInitEnd => "WARM_INIT_END",
            Self::ReconcileAndWarmInitEnd => "RECONCILE_AND_WARM_INIT_END",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BIND" => Some(Self::Bind),
            "VERIFY" => Some(Self::Verify),
            "VERIFY_AND_WARM_INIT_BEGIN" => Some(Self::VerifyAndWarmInitBegin),
            "VERIFY_AND_WARM_INIT_BEGIN_AND_END" => Some(Self::VerifyAndWarmInitBeginAndEnd),
            "WARM_INIT_END" => Some(Self::WarmInitEnd),
            "RECONCILE_AND_WARM_INIT_END" => Some(Self::ReconcileAndWarmInitEnd),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DevInitMode {
    /// This is the default device init mode.
    FastReconfig = 0,
    /// Device incurs a fast-reconfig reset with minimal traffic disruption
    ///
    /// Device incurs a hitless warm init. This incurs even lesser traffic
    Hitless = 1,
}
impl DevInitMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::FastReconfig => "FAST_RECONFIG",
            Self::Hitless => "HITLESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FAST_RECONFIG" => Some(Self::FastReconfig),
            "HITLESS" => Some(Self::Hitless),
            _ => None,
        }
    }
}
