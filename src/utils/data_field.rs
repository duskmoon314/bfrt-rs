use crate::bfrt::{
    data_field::{BoolArray, IntArray, StrArray, Value},
    DataField,
};

impl From<DataField> for Vec<DataField> {
    fn from(data_field: DataField) -> Self {
        vec![data_field]
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Stream(value.to_be_bytes().to_vec())
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Stream(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::FloatVal(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::StrVal(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::StrVal(value)
    }
}

impl From<Vec<u32>> for IntArray {
    fn from(value: Vec<u32>) -> Self {
        IntArray { val: value }
    }
}

impl From<Vec<u32>> for Value {
    fn from(value: Vec<u32>) -> Self {
        Value::IntArrVal(value.into())
    }
}

impl From<Vec<bool>> for BoolArray {
    fn from(value: Vec<bool>) -> Self {
        BoolArray { val: value }
    }
}

impl From<Vec<bool>> for Value {
    fn from(value: Vec<bool>) -> Self {
        Value::BoolArrVal(value.into())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::BoolVal(value)
    }
}

impl From<Vec<String>> for StrArray {
    fn from(value: Vec<String>) -> Self {
        StrArray { val: value }
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Self {
        Value::StrArrVal(value.into())
    }
}
