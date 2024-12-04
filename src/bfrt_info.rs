//! BFRT Info data structure.
//!
//! OpenTofino doesn't provide a protobuf file for BFRT Info JSON file.
//! This file is hand-crafted based on the JSON file generated by Intel SDE.
//!
//! Several fields may be omitted as they are not very important for author's use case.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BFRTInfo {
    pub schema_version: Option<String>,

    pub tables: Vec<Table>,

    #[serde(default)]
    pub learn_filters: Vec<LearnFilter>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Table {
    pub name: String,

    pub id: u32,

    pub table_type: TableType,

    pub size: u32,

    #[serde(default)]
    pub depends_on: Vec<u32>,

    pub has_const_default_action: Option<bool>,

    pub key: Vec<Key>,

    #[serde(default)]
    pub action_specs: Vec<Action>,

    #[serde(default)]
    pub data: Vec<Data>,

    pub supported_operations: Vec<SupportedOperation>,

    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum TableType {
    Counter,
    DevConfigure,
    LogDbgCnt,
    #[serde(rename = "MatchAction_Direct")]
    MatchActionDirect,
    Meter,
    MirrorCfg,
    PktgenAppCfg,
    PktgenPktBufferCfg,
    PktgenPortCfg,
    PortConfigure,
    PortFpIdxInfo,
    PortHdlInfo,
    PortMetadata,
    PortStat,
    PortStrInfo,
    PreEcmp,
    PreLag,
    PreMgid,
    PreNode,
    PrePort,
    PrePrune,
    Register,
    RegisterParam,
    SnapshotCfg,
    SnapshotData,
    SnapshotLiveness,
    SnapshotPhv,
    SnapshotTrigger,
    TblDbgCnt,
    TmCfg,
    TmCounterEgPort,
    TmCounterIgPort,
    TmCounterMirrorPortDpg,
    TmCounterPipe,
    TmCounterPool,
    TmCounterPortDpg,
    TmCounterPpg,
    TmCounterQueue,
    TmMirrorDpg,
    TmPipeCfg,
    TmPipeMulticastFifo,
    TmPipeSchedCfg,
    TmPoolApp,
    TmPoolAppPfc,
    TmPoolCfg,
    TmPoolColor,
    TmPoolSkid,
    TmPortBuffer,
    TmPortCfg,
    TmPortDpg,
    TmPortFlowcontrol,
    TmPortGroup,
    TmPortGroupCfg,
    TmPortSchedCfg,
    TmPortSchedShaping,
    TmPpgCfg,
    TmQueueBuffer,
    TmQueueCfg,
    TmQueueColor,
    TmQueueMap,
    TmQueueSchedCfg,
    TmQueueSchedShaping,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Key {
    pub id: u32,

    pub name: String,

    pub repeated: Option<bool>,

    pub mandatory: bool,

    pub match_type: MatchType,

    pub r#type: Type,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MatchType {
    Exact,
    Ternary,
    #[serde(rename = "LPM")]
    Lpm,
    Range,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Type {
    pub r#type: String,

    pub width: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Action {
    pub id: u32,

    pub name: String,

    pub action_scope: Option<ActionScope>,

    pub data: Vec<ActionData>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ActionScope {
    TableAndDefault,
    DefaultOnly,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ActionData {
    pub id: Option<u32>,

    pub name: Option<String>,

    pub repeated: Option<bool>,

    pub mandatory: bool,

    pub read_only: Option<bool>,

    pub r#type: Option<Type>,

    pub singleton: Option<Singleton>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Data {
    pub mandatory: bool,

    pub read_only: bool,

    pub singleton: Singleton,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Singleton {
    pub id: u32,

    pub name: String,

    pub repeated: bool,

    pub r#type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum SupportedOperation {
    Sync,
    SyncCounters,
    SyncRegisters,
    UpdateHitState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LearnFilter {
    pub name: String,

    pub id: u32,

    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Field {
    pub id: u32,

    pub name: String,

    pub repeated: bool,

    pub r#type: Type,
}
