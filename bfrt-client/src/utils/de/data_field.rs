use bfrt::bfrt::{data_field::Value, DataField};
use serde::de::{self, IntoDeserializer, SeqAccess};

use crate::DeserializeError;

pub fn from_data_field<'de, T>(data: &'de DataField) -> Result<T, DeserializeError>
where
    T: de::Deserialize<'de>,
{
    let deserializer = Deserializer::new(data);
    T::deserialize(deserializer)
}

pub struct Deserializer<'de> {
    data: &'de DataField,
}

impl<'de> Deserializer<'de> {
    pub fn new(data: &'de DataField) -> Self {
        Self { data }
    }

    pub fn parse_bool(&self) -> Result<bool, DeserializeError> {
        match &self.data.value {
            Some(Value::BoolVal(b)) => Ok(*b),
            Some(Value::Stream(bytes)) => {
                if bytes.len() == 1 {
                    Ok(bytes[0] != 0)
                } else {
                    Err(DeserializeError::ExpectedBool)
                }
            }
            _ => Err(DeserializeError::ExpectedBool),
        }
    }

    pub fn parse_i8(&self) -> Result<i8, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() == 1 {
                    Ok(bytes[0] as i8)
                } else {
                    Err(DeserializeError::ExpectedI8)
                }
            }
            _ => Err(DeserializeError::ExpectedI8),
        }
    }

    pub fn parse_i16(&self) -> Result<i16, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() > 2 {
                    Err(DeserializeError::ExpectedI16)
                } else {
                    let mut buf = [0; 2];
                    buf[2 - bytes.len()..].copy_from_slice(bytes);
                    Ok(i16::from_be_bytes(buf))
                }
            }
            _ => Err(DeserializeError::ExpectedI16),
        }
    }

    pub fn parse_i32(&self) -> Result<i32, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() > 4 {
                    Err(DeserializeError::ExpectedI32)
                } else {
                    let mut buf = [0; 4];
                    buf[4 - bytes.len()..].copy_from_slice(bytes);
                    Ok(i32::from_be_bytes(buf))
                }
            }
            _ => Err(DeserializeError::ExpectedI32),
        }
    }

    pub fn parse_i64(&self) -> Result<i64, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() > 8 {
                    Err(DeserializeError::ExpectedI64)
                } else {
                    let mut buf = [0; 8];
                    buf[8 - bytes.len()..].copy_from_slice(bytes);
                    Ok(i64::from_be_bytes(buf))
                }
            }
            _ => Err(DeserializeError::ExpectedI64),
        }
    }

    pub fn parse_u8(&self) -> Result<u8, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() == 1 {
                    Ok(bytes[0])
                } else {
                    Err(DeserializeError::ExpectedU8)
                }
            }
            _ => Err(DeserializeError::ExpectedU8),
        }
    }

    pub fn parse_u16(&self) -> Result<u16, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() > 2 {
                    Err(DeserializeError::ExpectedU16)
                } else {
                    let mut buf = [0; 2];
                    buf[2 - bytes.len()..].copy_from_slice(bytes);
                    Ok(u16::from_be_bytes(buf))
                }
            }
            _ => Err(DeserializeError::ExpectedU16),
        }
    }

    pub fn parse_u32(&self) -> Result<u32, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() > 4 {
                    Err(DeserializeError::ExpectedU32)
                } else {
                    let mut buf = [0; 4];
                    buf[4 - bytes.len()..].copy_from_slice(bytes);
                    Ok(u32::from_be_bytes(buf))
                }
            }
            _ => Err(DeserializeError::ExpectedU32),
        }
    }

    pub fn parse_u64(&self) -> Result<u64, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => {
                if bytes.len() > 8 {
                    Err(DeserializeError::ExpectedU64)
                } else {
                    let mut buf = [0; 8];
                    buf[8 - bytes.len()..].copy_from_slice(bytes);
                    Ok(u64::from_be_bytes(buf))
                }
            }
            _ => Err(DeserializeError::ExpectedU64),
        }
    }

    pub fn parse_f32(&self) -> Result<f32, DeserializeError> {
        match &self.data.value {
            Some(Value::FloatVal(f)) => Ok(*f),
            _ => Err(DeserializeError::ExpectedF32),
        }
    }

    pub fn parse_bytes(&self) -> Result<&[u8], DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => Ok(bytes),
            _ => Err(DeserializeError::ExpectedBytes),
        }
    }

    pub fn parse_bytes_buf(&self) -> Result<Vec<u8>, DeserializeError> {
        match &self.data.value {
            Some(Value::Stream(bytes)) => Ok(bytes.clone()),
            _ => Err(DeserializeError::ExpectedBytes),
        }
    }

    pub fn parse_string(&self) -> Result<String, DeserializeError> {
        match &self.data.value {
            Some(Value::StrVal(s)) => Ok(s.clone()),
            _ => Err(DeserializeError::ExpectedString),
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.parse_bytes_buf()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.data.value.is_none() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccessor {
            de: &self,
            index: 0,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccessor {
            de: &self,
            index: 0,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct SeqAccessor<'a, 'de: 'a> {
    de: &'a Deserializer<'de>,
    index: usize,
}

impl<'a, 'de> SeqAccess<'de> for SeqAccessor<'a, 'de> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match &self.de.data.value {
            Some(Value::Stream(bytes)) => {
                if self.index >= bytes.len() {
                    Ok(None)
                } else {
                    let byte = bytes[self.index];
                    self.index += 1;
                    seed.deserialize(byte.into_deserializer()).map(Some)
                }
            }
            _ => Err(DeserializeError::UnsupportedDataType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_unsigned() {
        let data = DataField {
            field_id: 1,
            value: Some(Value::Stream(vec![1, 2, 3, 4])),
        };

        let unsigned_32: u32 = from_data_field(&data).unwrap();
        assert_eq!(unsigned_32, 0x01020304);
    }

    #[test]
    fn deserialize_array() {
        let data = DataField {
            field_id: 1,
            value: Some(Value::Stream(vec![1, 2, 3, 4])),
        };

        let array: Vec<u8> = from_data_field(&data).unwrap();
        assert_eq!(array, vec![1, 2, 3, 4]);
    }

    #[test]
    fn deserialize_ipv4() {
        let data = DataField {
            field_id: 1,
            value: Some(Value::Stream(vec![192, 168, 1, 1])),
        };

        let ipv4: std::net::Ipv4Addr = from_data_field(&data).unwrap();
        assert_eq!(ipv4, std::net::Ipv4Addr::new(192, 168, 1, 1));
    }
}
