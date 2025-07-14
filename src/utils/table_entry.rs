use crate::bfrt::{table_entry::Value, TableData, TableEntry, TableFlags, TargetDevice};

impl TableEntry {
    /// Create a new `TableEntry` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modify the `table_id` field
    pub fn with_table_id(mut self, table_id: u32) -> Self {
        self.table_id = table_id;
        self
    }

    /// Modify the `data` field
    pub fn with_data(mut self, data: TableData) -> Self {
        self.data = Some(data);
        self
    }

    /// Modify the `is_default_entry` field
    pub fn with_is_default_entry(mut self, is_default_entry: bool) -> Self {
        self.is_default_entry = is_default_entry;
        self
    }

    /// Modify the `entry_tgt` field
    pub fn with_entry_tgt(mut self, entry_tgt: TargetDevice) -> Self {
        self.entry_tgt = Some(entry_tgt);
        self
    }

    /// Modify the `table_flags` field
    pub fn with_table_flags(mut self, table_flags: TableFlags) -> Self {
        self.table_flags = Some(table_flags);
        self
    }

    /// Modify the `value` field
    pub fn with_value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }
}
