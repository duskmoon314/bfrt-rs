use bfrt::{
    bfrt::{DataField, TableData},
    bfrt_info::LearnFilter,
};
use serde::de::{self, IntoDeserializer, MapAccess};

use crate::DeserializeError;

use super::data_field::from_data_field;

pub fn from_digest<'de, T>(
    info: &'de LearnFilter,
    data: &'de TableData,
) -> Result<T, DeserializeError>
where
    T: de::Deserialize<'de>,
{
    let mut deserializer = DigestDeserializer::new(info, data);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

pub struct DigestDeserializer<'de> {
    info: &'de LearnFilter,
    data: &'de TableData,
    index: usize,
}

impl<'de> DigestDeserializer<'de> {
    pub fn new(info: &'de LearnFilter, data: &'de TableData) -> Self {
        Self {
            info,
            data,
            index: 0,
        }
    }

    fn peek(&self) -> &DataField {
        &self.data.fields[self.index]
    }

    fn step(&mut self) {
        self.index += 1;
    }

    fn visit<T>(&mut self) -> Result<T, DeserializeError>
    where
        for<'a> T: de::Deserialize<'a>,
    {
        let value = self.peek();
        let value: T = from_data_field(value)?;

        self.step();
        Ok(value)
    }
}

impl<'de> de::Deserializer<'de> for &mut DigestDeserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.visit()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.visit()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.visit()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.visit()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.visit()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.visit()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.visit()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.visit()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.visit()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.visit()?)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.visit::<Vec<u8>>()?.as_slice())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.visit()?)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
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
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'de> MapAccess<'de> for DigestDeserializer<'de> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.index >= self.data.fields.len() {
            return Ok(None);
        }

        let field_name = self.info.fields[self.index].name.clone();
        seed.deserialize(field_name.into_deserializer()).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        if self.index >= self.data.fields.len() {
            return Err(DeserializeError::IndexOutOfBounds);
        }

        let value = seed.deserialize(super::data_field::Deserializer::new(
            &self.data.fields[self.index],
        ))?;
        self.index += 1;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bfrt::{
        bfrt::{DataField, TableData},
        bfrt_info::{Field, LearnFilter, Type},
    };

    #[test]
    fn deserialize_digest() {
        #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
        struct DigestA {
            dst_addr: u64,
            port: u16,
            src_addr: u64,
        }

        let learn_filter = LearnFilter {
            name: "digest_a_t".to_string(),
            id: 1,
            fields: vec![
                Field {
                    id: 1,
                    name: "dst_addr".to_string(),
                    repeated: false,
                    r#type: Type {
                        r#type: "bytes".to_string(),
                        width: Some(48),
                    },
                },
                Field {
                    id: 2,
                    name: "port".to_string(),
                    repeated: false,
                    r#type: Type {
                        r#type: "bytes".to_string(),
                        width: Some(9),
                    },
                },
                Field {
                    id: 3,
                    name: "src_addr".to_string(),
                    repeated: false,
                    r#type: Type {
                        r#type: "bytes".to_string(),
                        width: Some(48),
                    },
                },
            ],
        };

        let table_data = TableData {
            action_id: 1,
            fields: vec![
                DataField {
                    field_id: 1,
                    value: Some(bfrt::bfrt::data_field::Value::Stream(vec![
                        0, 1, 2, 3, 4, 5,
                    ])),
                },
                DataField {
                    field_id: 2,
                    value: Some(bfrt::bfrt::data_field::Value::Stream(vec![0, 1])),
                },
                DataField {
                    field_id: 3,
                    value: Some(bfrt::bfrt::data_field::Value::Stream(vec![
                        0, 1, 2, 3, 4, 5,
                    ])),
                },
            ],
        };

        let digest_a: DigestA = from_digest(&learn_filter, &table_data).unwrap();

        assert_eq!(digest_a.dst_addr, 0x000102030405);
        assert_eq!(digest_a.port, 0x01);
        assert_eq!(digest_a.src_addr, 0x000102030405);
    }

    #[test]
    fn deserialize_digest_ipv4() {
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        struct DigestIpv4 {
            dst_ip: std::net::Ipv4Addr,
        }

        let learn_filter = LearnFilter {
            name: "digest_ipv4".to_string(),
            id: 1234,
            fields: vec![Field {
                id: 1,
                name: "dst_ip".to_string(),
                repeated: false,
                r#type: Type {
                    r#type: "bytes".to_string(),
                    width: Some(32),
                },
            }],
        };

        let table_data = TableData {
            action_id: 0,
            fields: vec![DataField {
                field_id: 1,
                value: Some(bfrt::bfrt::data_field::Value::Stream(vec![0, 0, 0, 0])),
            }],
        };

        let digest_ipv4: DigestIpv4 = from_digest(&learn_filter, &table_data).unwrap();

        assert_eq!(digest_ipv4.dst_ip, std::net::Ipv4Addr::new(0, 0, 0, 0));
    }
}
