use crate::bfrt::{
    key_field::{Exact, Lpm, Optional, Range, Ternary},
    KeyField,
};

impl From<KeyField> for Vec<KeyField> {
    fn from(key_field: KeyField) -> Self {
        vec![key_field]
    }
}

impl<T: Into<Vec<u8>>> From<T> for Exact {
    fn from(value: T) -> Self {
        Exact {
            value: value.into(),
        }
    }
}

impl<T: Into<Vec<u8>>, U: Into<Vec<u8>>> From<(T, U)> for Ternary {
    fn from((value, mask): (T, U)) -> Self {
        Ternary {
            value: value.into(),
            mask: mask.into(),
        }
    }
}

impl<T: Into<Vec<u8>>, U: Into<i32>> From<(T, U)> for Lpm {
    fn from((value, prefix_len): (T, U)) -> Self {
        Lpm {
            value: value.into(),
            prefix_len: prefix_len.into(),
        }
    }
}

impl<T: Into<Vec<u8>>, U: Into<Vec<u8>>> From<(T, U)> for Range {
    fn from((low, high): (T, U)) -> Self {
        Range {
            low: low.into(),
            high: high.into(),
        }
    }
}

impl<T: Into<Vec<u8>>, U: Into<bool>> From<(T, U)> for Optional {
    fn from((value, is_valid): (T, U)) -> Self {
        Optional {
            value: value.into(),
            is_valid: is_valid.into(),
        }
    }
}
