#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Notifications {
    /// Enable learn digest notifications. These notifications are
    #[prost(bool, tag = "1")]
    pub enable_learn_notifications: bool,
    /// (device, P4-program) based so these will be triggered only after a
    /// client binds to a program.
    ///
    /// Enable idletimeout notifications. These are on per table basis and
    #[prost(bool, tag = "2")]
    pub enable_idletimeout_notifications: bool,
    /// hence (device, P4-Program) based so these will be triggered only
    /// after a client binds to a program.
    ///
    /// Enable port status change notifications. These notifications are
    #[prost(bool, tag = "3")]
    pub enable_port_status_change_notifications: bool,
    /// device based and so they will be triggered whether a client is
    /// bound to a program or not.
    ///
    /// Enable entry active notifications. These are on per table basis and
    #[prost(bool, tag = "4")]
    pub enable_entry_active_notifications: bool,
}
