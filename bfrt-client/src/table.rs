use std::borrow::{Borrow, BorrowMut};

use crate::{client::Client, ClientBasicError};

pub struct Table<T>
where
    T: Borrow<Client>,
{
    client: T,
}

impl<T: Borrow<Client>> Table<T> {
    /// Create a new table wrapper
    pub fn new(client: T) -> Self {
        Table { client }
    }

    pub fn get_by_name(
        &self,
        table_name: impl AsRef<str>,
    ) -> Option<crate::bfrt_info::table::Table> {
        let client: &Client = self.client.borrow();

        client.bfrt_info.as_ref().unwrap().get_table(table_name)
    }

    pub fn get_by_id(&self, table_id: u32) -> Option<crate::bfrt_info::table::Table> {
        let client: &Client = self.client.borrow();

        client.bfrt_info.as_ref().unwrap().get_table_by_id(table_id)
    }
}

impl<T: Borrow<Client> + BorrowMut<Client>> Table<T> {
    pub async fn get_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<Vec<bfrt::bfrt::TableEntry>, ClientBasicError> {
        let client: &mut Client = self.client.borrow_mut();

        let entities = table_entries
            .into_iter()
            .map(|entry| bfrt::bfrt::Entity {
                entity: Some(bfrt::bfrt::entity::Entity::TableEntry(entry)),
            })
            .collect();

        let mut res_stream = client.read(entities, target).await?;

        let mut entities: Vec<bfrt::bfrt::Entity> = Vec::new();

        while let Some(res) = res_stream.message().await? {
            entities.extend(res.entities);
        }

        let entries = entities
            .into_iter()
            .filter_map(|e| {
                if let Some(bfrt::bfrt::entity::Entity::TableEntry(entry)) = e.entity {
                    Some(entry)
                } else {
                    None
                }
            })
            .collect();

        Ok(entries)
    }

    pub async fn write_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        update_type: bfrt::bfrt::update::Type,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        let client: &mut Client = self.client.borrow_mut();

        let updates = table_entries
            .into_iter()
            .map(|entry| bfrt::bfrt::Update {
                r#type: update_type as i32,
                entity: Some(bfrt::bfrt::Entity {
                    entity: Some(bfrt::bfrt::entity::Entity::TableEntry(entry)),
                }),
            })
            .collect();

        client.write(updates, target).await?;

        Ok(())
    }

    pub async fn insert_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        self.write_entries(table_entries, bfrt::bfrt::update::Type::Insert, target)
            .await
    }

    pub async fn modify_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        self.write_entries(table_entries, bfrt::bfrt::update::Type::Modify, target)
            .await
    }

    pub async fn modify_inc_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        self.write_entries(table_entries, bfrt::bfrt::update::Type::ModifyInc, target)
            .await
    }

    pub async fn delete_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        self.write_entries(table_entries, bfrt::bfrt::update::Type::Delete, target)
            .await
    }

    pub async fn upsert_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        self.write_entries(
            table_entries,
            bfrt::bfrt::update::Type::InsertOrModify,
            target,
        )
        .await
    }

    pub async fn reset_entries(
        &mut self,
        table_entries: Vec<bfrt::bfrt::TableEntry>,
        target: Option<bfrt::bfrt::TargetDevice>,
    ) -> Result<(), ClientBasicError> {
        self.write_entries(table_entries, bfrt::bfrt::update::Type::Reset, target)
            .await
    }
}
