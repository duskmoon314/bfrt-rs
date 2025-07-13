/// ---
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    /// This is the default TargetDevice.
    /// If entry_tgt under TableEntry is specified, that takes precedence over this field
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<TargetDevice>,
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
    /// The write batch, comprising a list of Update operations.
    #[prost(message, repeated, tag = "3")]
    pub updates: ::prost::alloc::vec::Vec<Update>,
    #[prost(enumeration = "write_request::Atomicity", tag = "4")]
    pub atomicity: i32,
    #[prost(string, tag = "5")]
    pub p4_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WriteRequest`.
pub mod write_request;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    #[prost(message, repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// This is the default TargetDevice.
    /// If entry_tgt under TableEntry is specified, that takes precedence over this field
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<TargetDevice>,
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    #[prost(string, tag = "4")]
    pub p4_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub status: ::prost::alloc::vec::Vec<Error>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TargetDevice {
    #[prost(uint32, tag = "1")]
    pub device_id: u32,
    #[prost(uint32, tag = "2")]
    pub pipe_id: u32,
    #[prost(uint32, tag = "3")]
    pub direction: u32,
    /// More target-specific ids.
    #[prost(uint32, tag = "4")]
    pub prsr_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    #[prost(enumeration = "update::Type", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
}
/// Nested message and enum types in `Update`.
pub mod update;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(oneof = "entity::Entity", tags = "1, 2, 3, 4, 5, 6")]
    pub entity: ::core::option::Option<entity::Entity>,
}
/// Nested message and enum types in `Entity`.
pub mod entity;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleId {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(oneof = "handle_id::Value", tags = "2, 3")]
    pub value: ::core::option::Option<handle_id::Value>,
}
/// Nested message and enum types in `HandleId`.
pub mod handle_id;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableEntry {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<TableData>,
    #[prost(bool, tag = "4")]
    pub is_default_entry: bool,
    /// Deprecated, please use table_flags
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub table_read_flag: ::core::option::Option<TableReadFlag>,
    #[deprecated]
    #[prost(message, optional, tag = "6")]
    pub table_mod_inc_flag: ::core::option::Option<TableModIncFlag>,
    /// If entry_tgt is specified, all the fields of entry_tgt are used even if not explicitly set
    #[prost(message, optional, tag = "8")]
    pub entry_tgt: ::core::option::Option<TargetDevice>,
    #[prost(message, optional, tag = "9")]
    pub table_flags: ::core::option::Option<TableFlags>,
    #[prost(oneof = "table_entry::Value", tags = "2, 7")]
    pub value: ::core::option::Option<table_entry::Value>,
}
/// Nested message and enum types in `TableEntry`.
pub mod table_entry;
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TableUsage {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(uint32, tag = "2")]
    pub usage: u32,
    /// Deprecated, please use table_flags
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub table_read_flag: ::core::option::Option<TableReadFlag>,
    #[prost(message, optional, tag = "4")]
    pub table_flags: ::core::option::Option<TableFlags>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableAttribute {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(oneof = "table_attribute::Attribute", tags = "2, 3, 4, 5, 6, 7, 8, 9")]
    pub attribute: ::core::option::Option<table_attribute::Attribute>,
}
/// Nested message and enum types in `TableAttribute`.
pub mod table_attribute;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableOperation {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(string, tag = "2")]
    pub table_operations_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableData {
    #[prost(uint32, tag = "1")]
    pub action_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<DataField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataField {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    /// All data fields are dealt with using a byte stream except for float
    /// values. Float values are used for data fields for LPF and WRED table
    #[prost(oneof = "data_field::Value", tags = "2, 3, 4, 5, 6, 7, 8, 9")]
    pub value: ::core::option::Option<data_field::Value>,
}
/// Nested message and enum types in `DataField`.
pub mod data_field;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableKey {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<KeyField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyField {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    #[prost(oneof = "key_field::MatchType", tags = "2, 3, 4, 5, 6")]
    pub match_type: ::core::option::Option<key_field::MatchType>,
}
/// Nested message and enum types in `KeyField`.
pub mod key_field;
/// Deprecated, please use TableFlags
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TableReadFlag {
    #[prost(bool, tag = "1")]
    pub from_hw: bool,
    #[prost(bool, tag = "2")]
    pub key_only: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TableFlags {
    #[prost(bool, tag = "1")]
    pub from_hw: bool,
    #[prost(bool, tag = "2")]
    pub key_only: bool,
    #[prost(bool, tag = "3")]
    pub mod_del: bool,
    #[prost(bool, tag = "4")]
    pub reset_ttl: bool,
    #[prost(bool, tag = "5")]
    pub reset_stats: bool,
}
/// Deprecated, please use TableFlags
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TableModIncFlag {
    #[prost(enumeration = "table_mod_inc_flag::Type", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `TableModIncFlag`.
pub mod table_mod_inc_flag;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyFieldMask {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub mask: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynKeyMask {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<KeyFieldMask>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DynHashing {
    #[prost(uint32, tag = "1")]
    pub alg: u32,
    #[prost(uint64, tag = "2")]
    pub seed: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ByteCountAdj {
    #[prost(int32, tag = "1")]
    pub byte_count_adjust: i32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct IdleTable {
    #[prost(uint32, tag = "1")]
    pub ttl_query_interval: u32,
    #[prost(uint32, tag = "2")]
    pub max_ttl: u32,
    #[prost(uint32, tag = "3")]
    pub min_ttl: u32,
    #[prost(enumeration = "idle_table::IdleTableMode", tag = "4")]
    pub idle_table_mode: i32,
    #[prost(bool, tag = "5")]
    pub enable: bool,
}
/// Nested message and enum types in `IdleTable`.
pub mod idle_table;
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct StatePullIntvl {
    #[prost(uint32, tag = "1")]
    pub intvl_val: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PortStatusChg {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Mode {
    #[prost(uint64, tag = "3")]
    pub args: u64,
    #[prost(oneof = "mode::Scope", tags = "1, 2")]
    pub scope: ::core::option::Option<mode::Scope>,
}
/// Nested message and enum types in `Mode`.
pub mod mode;
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreGlobalRid {
    #[prost(uint32, tag = "1")]
    pub global_rid: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PrePortProtection {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreFastFailover {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreMaxNodesBeforeYield {
    #[prost(uint32, tag = "1")]
    pub count: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreMaxNodeThreshold {
    #[prost(uint32, tag = "1")]
    pub node_count: u32,
    #[prost(uint32, tag = "2")]
    pub port_lag_count: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreDeviceConfig {
    #[prost(message, optional, tag = "1")]
    pub pre_global_rid: ::core::option::Option<PreGlobalRid>,
    #[prost(message, optional, tag = "2")]
    pub pre_port_protection: ::core::option::Option<PrePortProtection>,
    #[prost(message, optional, tag = "3")]
    pub pre_fast_failover: ::core::option::Option<PreFastFailover>,
    #[prost(message, optional, tag = "4")]
    pub pre_max_nodes_before_yield: ::core::option::Option<PreMaxNodesBeforeYield>,
    #[prost(message, optional, tag = "5")]
    pub pre_max_node_threshold: ::core::option::Option<PreMaxNodeThreshold>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EntryScope {
    #[prost(message, optional, tag = "1")]
    pub gress_scope: ::core::option::Option<Mode>,
    #[prost(message, optional, tag = "2")]
    pub pipe_scope: ::core::option::Option<Mode>,
    #[prost(message, optional, tag = "3")]
    pub prsr_scope: ::core::option::Option<Mode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectId {
    #[prost(uint32, tag = "3")]
    pub id: u32,
    #[prost(oneof = "object_id::Object", tags = "1, 2")]
    pub object: ::core::option::Option<object_id::Object>,
}
/// Nested message and enum types in `ObjectId`.
pub mod object_id;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMessageRequest {
    #[prost(uint32, tag = "1")]
    pub client_id: u32,
    #[prost(oneof = "stream_message_request::Update", tags = "2, 3")]
    pub update: ::core::option::Option<stream_message_request::Update>,
}
/// Nested message and enum types in `StreamMessageRequest`.
pub mod stream_message_request;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    #[deprecated]
    #[prost(bool, tag = "1")]
    pub is_master: bool,
    /// Master for Warm Init messages.
    /// Deprecated and not needed anymore.
    /// Keeping for backward compatibility.
    ///
    /// Device ID
    #[prost(uint32, tag = "2")]
    pub device_id: u32,
    /// Contains which notifications need to be
    #[prost(message, optional, tag = "3")]
    pub notifications: ::core::option::Option<subscribe::Notifications>,
    /// enabled for this client. Default value of
    /// these notifications are false.
    ///
    /// The controller doesn't populate this field.
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<super::google::rpc::Status>,
}
/// Nested message and enum types in `Subscribe`.
pub mod subscribe;
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DigestListAck {
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[prost(uint32, tag = "2")]
    pub list_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMessageResponse {
    #[prost(oneof = "stream_message_response::Update", tags = "1, 2, 3, 4, 5")]
    pub update: ::core::option::Option<stream_message_response::Update>,
}
/// Nested message and enum types in `StreamMessageResponse`.
pub mod stream_message_response;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DigestList {
    /// Identifies the digest extern instance
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[prost(uint32, tag = "2")]
    pub list_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<TableData>,
    #[prost(message, optional, tag = "4")]
    pub target: ::core::option::Option<TargetDevice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdleTimeoutNotification {
    /// Only "key" fields are required to be set in each TableEntry.
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<TargetDevice>,
    #[prost(message, optional, tag = "2")]
    pub table_entry: ::core::option::Option<TableEntry>,
    #[prost(enumeration = "idle_timeout_notification::NotificationType", tag = "3")]
    pub r#type: i32,
}
/// Nested message and enum types in `IdleTimeoutNotification`.
pub mod idle_timeout_notification;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortStatusChgNotification {
    /// Only "key" fields are required to be set in each TableEntry.
    #[prost(message, optional, tag = "1")]
    pub table_entry: ::core::option::Option<TableEntry>,
    #[prost(bool, tag = "2")]
    pub port_up: bool,
}
/// ---
///
/// SetForwardingPipelineConfig RPC takes in this message. It should contain
/// details of the entire device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetForwardingPipelineConfigRequest {
    /// Device ID
    #[prost(uint32, tag = "1")]
    pub device_id: u32,
    /// Client ID
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
    /// action
    #[prost(
        enumeration = "set_forwarding_pipeline_config_request::Action",
        tag = "3"
    )]
    pub action: i32,
    /// warm init mode. Fast reconfig or Hitless
    #[prost(
        enumeration = "set_forwarding_pipeline_config_request::DevInitMode",
        tag = "4"
    )]
    pub dev_init_mode: i32,
    /// The base path where the config is wished to be
    #[prost(string, tag = "5")]
    pub base_path: ::prost::alloc::string::String,
    /// stored. If empty, then current directory is used
    ///
    /// Device's config
    #[prost(message, repeated, tag = "6")]
    pub config: ::prost::alloc::vec::Vec<ForwardingPipelineConfig>,
}
/// Nested message and enum types in `SetForwardingPipelineConfigRequest`.
pub mod set_forwarding_pipeline_config_request;
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SetForwardingPipelineConfigResponse {
    #[prost(enumeration = "SetForwardingPipelineConfigResponseType", tag = "1")]
    pub set_forwarding_pipeline_config_response_type: i32,
}
/// This message contains config of a SINGLE program. The reason config is a
/// repeated field in the SetForwardingPipelineConfigRequest is because a
/// device can have multiple programs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingPipelineConfig {
    /// P4 program name
    #[prost(string, tag = "1")]
    pub p4_name: ::prost::alloc::string::String,
    /// BF-RT info json file contents
    #[prost(bytes = "vec", tag = "2")]
    pub bfruntime_info: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub profiles: ::prost::alloc::vec::Vec<forwarding_pipeline_config::Profile>,
}
/// Nested message and enum types in `ForwardingPipelineConfig`.
pub mod forwarding_pipeline_config;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonP4Config {
    #[prost(bytes = "vec", tag = "1")]
    pub bfruntime_info: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get config of the entire device. Any client can issue this
/// request
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetForwardingPipelineConfigRequest {
    #[prost(uint32, tag = "1")]
    pub device_id: u32,
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
}
/// Config of the entire device
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetForwardingPipelineConfigResponse {
    /// P4 info
    #[prost(message, repeated, tag = "1")]
    pub config: ::prost::alloc::vec::Vec<ForwardingPipelineConfig>,
    /// Non-P4 info
    #[prost(message, optional, tag = "2")]
    pub non_p4_config: ::core::option::Option<NonP4Config>,
}
/// Error message used to report a single P4-entity error for a Write RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// gRPC canonical error code (see
    /// github.com/grpc/grpc-go/blob/master/codes/codes.go)
    #[prost(int32, tag = "1")]
    pub canonical_code: i32,
    /// Detailed error message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Target and architecture specific space to which this error belongs.
    /// We encourage using triplet: <target>-<arch>-<vendor>,
    /// e.g."targetX-psa-vendor1" or "targetY-psa-vendor2".
    #[prost(string, tag = "3")]
    pub space: ::prost::alloc::string::String,
    /// Numeric code drawn from target-specific error space above.
    #[prost(int32, tag = "4")]
    pub code: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetForwardingPipelineConfigResponseType {
    /// WARM_INIT_STARTED indicates a successful
    WarmInitStarted = 0,
    /// WARM_INIT_FINISHED indicates a successful
    WarmInitFinished = 1,
}
impl SetForwardingPipelineConfigResponseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::WarmInitStarted => "WARM_INIT_STARTED",
            Self::WarmInitFinished => "WARM_INIT_FINISHED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WARM_INIT_STARTED" => Some(Self::WarmInitStarted),
            "WARM_INIT_FINISHED" => Some(Self::WarmInitFinished),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod bf_runtime_client;
/// Generated server implementations.
pub mod bf_runtime_server;
