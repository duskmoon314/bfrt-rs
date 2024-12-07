//! BFRTInfo learn helper struct and methods

use std::ops::Deref;

use crate::{utils::de::table_data::from_digest, DeserializeError};

#[derive(Clone, Debug)]
pub struct Learn<T = ()> {
    learn: bfrt::bfrt_info::LearnFilter,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Deref for Learn<T> {
    type Target = bfrt::bfrt_info::LearnFilter;

    fn deref(&self) -> &Self::Target {
        &self.learn
    }
}

impl<T> Learn<T> {
    pub fn new(learn: &bfrt::bfrt_info::LearnFilter) -> Self {
        Learn {
            learn: learn.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn parse_data<'de>(
        &'de self,
        data: &'de [bfrt::bfrt::TableData],
    ) -> Result<Vec<T>, DeserializeError>
    where
        T: serde::Deserialize<'de>,
    {
        data.iter()
            .map(|d| from_digest(&self.learn, d))
            .collect::<Result<Vec<_>, _>>()
    }
}
