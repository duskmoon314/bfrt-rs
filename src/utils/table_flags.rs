use std::ops::BitOr;

use crate::bfrt::TableFlags;

/// Convenient operations for table flags.
///
/// # Example
///
/// ```
/// use bfrt::bfrt::TableFlags;
///
/// let flags = TableFlags::FROM_HW | TableFlags::KEY_ONLY;
/// assert!(flags.from_hw);
/// assert!(flags.key_only);
///
/// let flags = TableFlags::new()
///     .with_from_hw(true)
///     .with_key_only(false);
/// assert!(flags.from_hw);
/// assert!(!flags.key_only);
/// ```
impl TableFlags {
    pub const FROM_HW: TableFlags = TableFlags {
        from_hw: true,
        key_only: false,
        mod_del: false,
        reset_ttl: false,
        reset_stats: false,
    };

    pub const KEY_ONLY: TableFlags = TableFlags {
        from_hw: false,
        key_only: true,
        mod_del: false,
        reset_ttl: false,
        reset_stats: false,
    };

    pub const MOD_DEL: TableFlags = TableFlags {
        from_hw: false,
        key_only: false,
        mod_del: true,
        reset_ttl: false,
        reset_stats: false,
    };

    pub const RESET_TTL: TableFlags = TableFlags {
        from_hw: false,
        key_only: false,
        mod_del: false,
        reset_ttl: true,
        reset_stats: false,
    };

    pub const RESET_STATS: TableFlags = TableFlags {
        from_hw: false,
        key_only: false,
        mod_del: false,
        reset_ttl: false,
        reset_stats: true,
    };

    /// Create a new `TableFlags` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modify the `from_hw` field
    pub fn with_from_hw(mut self, from_hw: bool) -> Self {
        self.from_hw = from_hw;
        self
    }

    /// Modify the `key_only` field
    pub fn with_key_only(mut self, key_only: bool) -> Self {
        self.key_only = key_only;
        self
    }

    /// Modify the `mod_del` field
    pub fn with_mod_del(mut self, mod_del: bool) -> Self {
        self.mod_del = mod_del;
        self
    }

    /// Modify the `reset_ttl` field
    pub fn with_reset_ttl(mut self, reset_ttl: bool) -> Self {
        self.reset_ttl = reset_ttl;
        self
    }

    /// Modify the `reset_stats` field
    pub fn with_reset_stats(mut self, reset_stats: bool) -> Self {
        self.reset_stats = reset_stats;
        self
    }
}

impl BitOr for TableFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            from_hw: self.from_hw | rhs.from_hw,
            key_only: self.key_only | rhs.key_only,
            mod_del: self.mod_del | rhs.mod_del,
            reset_ttl: self.reset_ttl | rhs.reset_ttl,
            reset_stats: self.reset_stats | rhs.reset_stats,
        }
    }
}
