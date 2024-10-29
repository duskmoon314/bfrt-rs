#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Attribute {
    #[prost(message, tag = "2")]
    IdleTable(super::IdleTable),
    #[prost(message, tag = "3")]
    EntryScope(super::EntryScope),
    #[prost(message, tag = "4")]
    DynKeyMask(super::DynKeyMask),
    #[prost(message, tag = "5")]
    DynHashing(super::DynHashing),
    #[prost(message, tag = "6")]
    ByteCountAdj(super::ByteCountAdj),
    #[prost(message, tag = "7")]
    PortStatusNotify(super::PortStatusChg),
    #[prost(message, tag = "8")]
    IntvlMs(super::StatePullIntvl),
    #[prost(message, tag = "9")]
    PreDeviceConfig(super::PreDeviceConfig),
}
