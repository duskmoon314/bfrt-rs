use std::collections::HashMap;

use bfrt::bfrt_info::{self, LearnFilter};

pub mod learn;
pub mod table;

/// Wrapper for real BFRT info, provides utility methods
#[derive(Debug, Clone)]
pub struct BFRTInfo {
    // bfrt_info: bfrt_info::BFRTInfo,
    p4_name: String,

    table_map: HashMap<String, bfrt_info::Table>,

    learn_map: HashMap<String, bfrt_info::LearnFilter>,

    table_id_map: HashMap<u32, bfrt_info::Table>,

    learn_id_map: HashMap<u32, bfrt_info::LearnFilter>,
}

impl BFRTInfo {
    pub fn new(
        p4_name: String,
        p4_info: impl AsRef<[u8]>,
        non_p4_info: Option<impl AsRef<[u8]>>,
    ) -> Result<Self, serde_json::Error> {
        let p4_info: bfrt_info::BFRTInfo = serde_json::from_slice(p4_info.as_ref())?;

        let mut table_map = HashMap::new();
        let mut learn_map = HashMap::new();
        let mut table_id_map = HashMap::new();
        let mut learn_id_map = HashMap::new();

        for table in &p4_info.tables {
            // bfrt info has table names prefixed with "pipe."
            // This is not the same as data names, which are not prefixed
            table_map.insert(table.name.replacen("pipe.", "", 1), table.clone());
            table_id_map.insert(table.id, table.clone());
        }

        for learn in &p4_info.learn_filters {
            learn_map.insert(learn.name.replacen("pipe.", "", 1), learn.clone());
            learn_id_map.insert(learn.id, learn.clone());
        }

        if let Some(non_p4_info) = non_p4_info
            .map(|info| serde_json::from_slice::<bfrt_info::BFRTInfo>(info.as_ref()))
            .transpose()?
        {
            for table in &non_p4_info.tables {
                table_map.insert(table.name.clone(), table.clone());
                table_id_map.insert(table.id, table.clone());
            }

            for learn in &non_p4_info.learn_filters {
                learn_map.insert(learn.name.clone(), learn.clone());
                learn_id_map.insert(learn.id, learn.clone());
            }
        }

        Ok(Self {
            // bfrt_info,
            p4_name,
            table_map,
            learn_map,
            table_id_map,
            learn_id_map,
        })
    }

    // pub fn bfrt_info(&self) -> &bfrt_info::BFRTInfo {
    //     &self.bfrt_info
    // }

    pub fn p4_name(&self) -> &String {
        &self.p4_name
    }

    pub fn table_map(&self) -> &HashMap<String, bfrt_info::Table> {
        &self.table_map
    }

    pub fn learn_map(&self) -> &HashMap<String, LearnFilter> {
        &self.learn_map
    }

    pub fn table_id_map(&self) -> &HashMap<u32, bfrt_info::Table> {
        &self.table_id_map
    }

    pub fn learn_id_map(&self) -> &HashMap<u32, LearnFilter> {
        &self.learn_id_map
    }

    pub fn get_table(&self, name: impl AsRef<str>) -> Option<table::Table> {
        self.table_map.get(name.as_ref()).map(table::Table::new)
    }

    pub fn get_learn<T>(&self, name: impl AsRef<str>) -> Option<learn::Learn<T>> {
        self.learn_map.get(name.as_ref()).map(learn::Learn::new)
    }

    pub fn get_table_by_id(&self, id: u32) -> Option<table::Table> {
        self.table_id_map.get(&id).map(table::Table::new)
    }

    pub fn get_learn_by_id<T>(&self, id: u32) -> Option<learn::Learn<T>> {
        self.learn_id_map.get(&id).map(learn::Learn::new)
    }
}
