use serde::de;

use crate::DeserializeError;

pub mod data_field;
pub mod table_data;

impl de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        DeserializeError::Custom {
            msg: msg.to_string(),
        }
    }
}
