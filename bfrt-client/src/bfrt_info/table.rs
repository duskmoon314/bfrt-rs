//! BFRTInfo table helper struct and methods

use std::{collections::HashMap, ops::Deref};

use bfrt::bfrt::key_field;

use crate::{MakeTableDataError, MakeTableKeyError};

/// Key value
///
/// This enum is used to represent the second value of a key
pub enum KeyValue {
    /// A byte vector
    Bytes(Vec<u8>),

    /// A 32-bit integer
    I32(i32),

    /// A boolean
    Bool(bool),
}

impl From<Vec<u8>> for KeyValue {
    fn from(v: Vec<u8>) -> Self {
        KeyValue::Bytes(v)
    }
}

impl From<i32> for KeyValue {
    fn from(v: i32) -> Self {
        KeyValue::I32(v)
    }
}

impl From<bool> for KeyValue {
    fn from(v: bool) -> Self {
        KeyValue::Bool(v)
    }
}

#[derive(Clone, Debug)]
pub struct DataValue(bfrt::bfrt::data_field::Value);

impl Deref for DataValue {
    type Target = bfrt::bfrt::data_field::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DataValue> for bfrt::bfrt::data_field::Value {
    fn from(v: DataValue) -> Self {
        v.0
    }
}

impl From<Vec<u8>> for DataValue {
    fn from(v: Vec<u8>) -> Self {
        DataValue(bfrt::bfrt::data_field::Value::Stream(v))
    }
}

impl From<f32> for DataValue {
    fn from(v: f32) -> Self {
        DataValue(bfrt::bfrt::data_field::Value::FloatVal(v))
    }
}

impl From<String> for DataValue {
    fn from(v: String) -> Self {
        DataValue(bfrt::bfrt::data_field::Value::StrVal(v))
    }
}

impl From<&str> for DataValue {
    fn from(v: &str) -> Self {
        DataValue(bfrt::bfrt::data_field::Value::StrVal(v.to_string()))
    }
}

impl From<bool> for DataValue {
    fn from(v: bool) -> Self {
        DataValue(bfrt::bfrt::data_field::Value::BoolVal(v))
    }
}

/// BFRTInfo table helper struct
///
/// Wraps around a [`bfrt::bfrt_info::Table`] object and provides helper methods
#[derive(Clone, Debug)]
pub struct Table {
    // table: &'a bfrt::bfrt_info::Table,
    table: bfrt::bfrt_info::Table,

    key_map: HashMap<String, bfrt::bfrt_info::Key>,
    action_map: HashMap<String, bfrt::bfrt_info::Action>,
}

impl Deref for Table {
    type Target = bfrt::bfrt_info::Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl Table {
    /// Create a new `Table`
    pub fn new(table: &bfrt::bfrt_info::Table) -> Self {
        let mut key_map = HashMap::new();
        let mut action_map = HashMap::new();

        for key in table.key.iter() {
            key_map.insert(key.name.clone(), key.clone());
        }

        for action in table.action_specs.iter() {
            action_map.insert(action.name.clone(), action.clone());
        }

        Table {
            table: table.clone(),
            key_map,
            action_map,
        }
    }

    /// Make a new key with exact type
    pub fn make_key_exact(
        &self,
        name: impl AsRef<str>,
        value: impl Into<bfrt::bfrt::key_field::Exact>,
    ) -> Result<bfrt::bfrt::KeyField, MakeTableKeyError> {
        use bfrt::bfrt_info::MatchType;

        let name = name.as_ref();

        let key_field = self
            .key_map
            .get(name)
            .ok_or(MakeTableKeyError::UnexistedField {
                field_name: name.to_string(),
            })?;

        let match_type = match key_field.match_type {
            MatchType::Exact => Ok(key_field::MatchType::Exact(value.into())),
            _ => Err(MakeTableKeyError::WrongMatchType),
        }?;

        Ok(bfrt::bfrt::KeyField {
            field_id: key_field.id,
            match_type: Some(match_type),
        })
    }

    /// Make a new key with ternary type
    pub fn make_key_ternary(
        &self,
        name: impl AsRef<str>,
        value: impl Into<bfrt::bfrt::key_field::Ternary>,
    ) -> Result<bfrt::bfrt::KeyField, MakeTableKeyError> {
        use bfrt::bfrt_info::MatchType;

        let name = name.as_ref();

        let key_field = self
            .key_map
            .get(name)
            .ok_or(MakeTableKeyError::UnexistedField {
                field_name: name.to_string(),
            })?;

        let match_type = match key_field.match_type {
            MatchType::Ternary => Ok(key_field::MatchType::Ternary(value.into())),
            _ => Err(MakeTableKeyError::WrongMatchType),
        }?;

        Ok(bfrt::bfrt::KeyField {
            field_id: key_field.id,
            match_type: Some(match_type),
        })
    }

    /// Make a new key with LPM type
    pub fn make_key_lpm(
        &self,
        name: impl AsRef<str>,
        value: impl Into<bfrt::bfrt::key_field::Lpm>,
    ) -> Result<bfrt::bfrt::KeyField, MakeTableKeyError> {
        use bfrt::bfrt_info::MatchType;

        let name = name.as_ref();

        let key_field = self
            .key_map
            .get(name)
            .ok_or(MakeTableKeyError::UnexistedField {
                field_name: name.to_string(),
            })?;

        let match_type = match key_field.match_type {
            MatchType::Lpm => Ok(key_field::MatchType::Lpm(value.into())),
            _ => Err(MakeTableKeyError::WrongMatchType),
        }?;

        Ok(bfrt::bfrt::KeyField {
            field_id: key_field.id,
            match_type: Some(match_type),
        })
    }

    /// Make a new key with range type
    pub fn make_key_range(
        &self,
        name: impl AsRef<str>,
        value: impl Into<bfrt::bfrt::key_field::Range>,
    ) -> Result<bfrt::bfrt::KeyField, MakeTableKeyError> {
        use bfrt::bfrt_info::MatchType;

        let name = name.as_ref();

        let key_field = self
            .key_map
            .get(name)
            .ok_or(MakeTableKeyError::UnexistedField {
                field_name: name.to_string(),
            })?;

        let match_type = match key_field.match_type {
            MatchType::Range => Ok(key_field::MatchType::Range(value.into())),
            _ => Err(MakeTableKeyError::WrongMatchType),
        }?;

        Ok(bfrt::bfrt::KeyField {
            field_id: key_field.id,
            match_type: Some(match_type),
        })
    }

    /// Make a new key with optional type
    pub fn make_key_optional(
        &self,
        name: impl AsRef<str>,
        value: impl Into<bfrt::bfrt::key_field::Optional>,
    ) -> Result<bfrt::bfrt::KeyField, MakeTableKeyError> {
        use bfrt::bfrt_info::MatchType;

        let name = name.as_ref();

        let key_field = self
            .key_map
            .get(name)
            .ok_or(MakeTableKeyError::UnexistedField {
                field_name: name.to_string(),
            })?;

        let match_type = match key_field.match_type {
            MatchType::Optional => Ok(key_field::MatchType::Optional(value.into())),
            _ => Err(MakeTableKeyError::WrongMatchType),
        }?;

        Ok(bfrt::bfrt::KeyField {
            field_id: key_field.id,
            match_type: Some(match_type),
        })
    }

    /// Make a new key
    pub fn make_key(
        &self,
        name: impl AsRef<str>,
        first_value: impl Into<Vec<u8>>,
        second_value: Option<impl Into<KeyValue>>,
    ) -> Result<bfrt::bfrt::KeyField, MakeTableKeyError> {
        use bfrt::bfrt::key_field::{self, *};
        use bfrt::bfrt_info::MatchType;

        let name = name.as_ref();

        let key_field = self
            .key_map
            .get(name)
            .ok_or(MakeTableKeyError::UnexistedField {
                field_name: name.to_string(),
            })?;

        let match_type = match key_field.match_type {
            MatchType::Exact => key_field::MatchType::Exact(Exact {
                value: first_value.into(),
            }),
            MatchType::Ternary => {
                let second = second_value
                    .ok_or(MakeTableKeyError::MissingSecondValue)
                    .map(|v| v.into())?;

                match second {
                    KeyValue::Bytes(mask) => key_field::MatchType::Ternary(Ternary {
                        value: first_value.into(),
                        mask,
                    }),
                    _ => return Err(MakeTableKeyError::ExpectedBytes),
                }
            }
            MatchType::Lpm => {
                let second = second_value
                    .ok_or(MakeTableKeyError::MissingSecondValue)
                    .map(|v| v.into())?;

                match second {
                    KeyValue::I32(prefix_len) => key_field::MatchType::Lpm(Lpm {
                        value: first_value.into(),
                        prefix_len,
                    }),
                    _ => return Err(MakeTableKeyError::ExpectedI32),
                }
            }
            MatchType::Range => {
                let second = second_value
                    .ok_or(MakeTableKeyError::MissingSecondValue)
                    .map(|v| v.into())?;

                match second {
                    KeyValue::Bytes(high) => key_field::MatchType::Range(Range {
                        low: first_value.into(),
                        high,
                    }),
                    _ => return Err(MakeTableKeyError::ExpectedBytes),
                }
            }
            MatchType::Optional => {
                let second = second_value
                    .ok_or(MakeTableKeyError::MissingSecondValue)
                    .map(|v| v.into())?;

                match second {
                    KeyValue::Bool(is_valid) => key_field::MatchType::Optional(Optional {
                        value: first_value.into(),
                        is_valid,
                    }),
                    _ => return Err(MakeTableKeyError::ExpectedBool),
                }
            }
            _ => return Err(MakeTableKeyError::UnsupportedMatchType),
        };

        Ok(bfrt::bfrt::KeyField {
            field_id: key_field.id,
            match_type: Some(match_type),
        })
    }

    pub fn make_data(
        &self,
        action_name: Option<impl AsRef<str>>,
        data_list: &[(impl AsRef<str>, DataValue)],
    ) -> Result<bfrt::bfrt::TableData, MakeTableDataError> {
        if let Some(s) = action_name {
            // Make an ActionData

            let action =
                self.action_map
                    .get(s.as_ref())
                    .ok_or(MakeTableDataError::UnexistedAction {
                        action_name: s.as_ref().to_string(),
                    })?;

            let data_fields = data_list
                .iter()
                .map(|(field_name, field_value)| {
                    let field = action
                        .data
                        .iter()
                        .find(|f| f.name.as_ref().map(|s| s.as_ref()) == Some(field_name.as_ref()))
                        .ok_or(MakeTableDataError::UnexistedField {
                            field_name: field_name.as_ref().to_string(),
                        });

                    field.map(|f| bfrt::bfrt::DataField {
                        field_id: f.id.expect("Action Data ID is None"),
                        value: Some(field_value.clone().into()),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(bfrt::bfrt::TableData {
                action_id: action.id,
                fields: data_fields,
            })
        } else {
            // Make a TableData

            let action_id = 0;

            let data_fields = data_list
                .iter()
                .map(|(field_name, field_value)| {
                    let field = self
                        .data
                        .iter()
                        .find(|f| {
                            f.singleton
                                .as_ref()
                                .expect("Only support singleton for now")
                                .name
                                == field_name.as_ref()
                        })
                        .ok_or(MakeTableDataError::UnexistedField {
                            field_name: field_name.as_ref().to_string(),
                        });

                    field.map(|f| bfrt::bfrt::DataField {
                        field_id: f
                            .singleton
                            .as_ref()
                            .expect("Only support singleton for now")
                            .id,
                        value: Some(field_value.clone().into()),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(bfrt::bfrt::TableData {
                action_id,
                fields: data_fields,
            })
        }
    }

    pub fn make_entry(
        &self,
        keys: Vec<bfrt::bfrt::KeyField>,
        data: Option<bfrt::bfrt::TableData>,
        flags: Option<bfrt::bfrt::TableFlags>,
    ) -> bfrt::bfrt::TableEntry {
        bfrt::bfrt::TableEntry {
            table_id: self.id,
            data,
            value: Some(bfrt::bfrt::table_entry::Value::Key(bfrt::bfrt::TableKey {
                fields: keys,
            })),
            table_flags: flags,
            ..Default::default()
        }
    }

    pub fn make_port_status_change_attr(&self, enable: bool) -> bfrt::bfrt::TableAttribute {
        bfrt::bfrt::TableAttribute {
            table_id: self.id,
            attribute: Some(bfrt::bfrt::table_attribute::Attribute::PortStatusNotify(
                bfrt::bfrt::PortStatusChg { enable },
            )),
        }
    }
}
