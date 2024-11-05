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

    pub fn get_by_name(&self, table_name: impl AsRef<str>) -> Option<&bfrt::bfrt_info::Table> {
        let client: &Client = self.client.borrow();

        client.bfrt_info.as_ref().unwrap().get_table(table_name)
    }

    pub fn get_by_id(&self, table_id: u32) -> Option<&bfrt::bfrt_info::Table> {
        let client: &Client = self.client.borrow();

        client.bfrt_info.as_ref().unwrap().get_table_by_id(table_id)
    }
}

impl<T: Borrow<Client> + BorrowMut<Client>> Table<T> {
    pub async fn get_entry(
        &mut self,
        table_entry: bfrt::bfrt::TableEntry,
    ) -> Result<Vec<bfrt::bfrt::TableEntry>, ClientBasicError> {
        let client: &mut Client = self.client.borrow_mut();

        let entity = bfrt::bfrt::Entity {
            entity: Some(bfrt::bfrt::entity::Entity::TableEntry(table_entry)),
        };

        let mut res_stream = client.read(vec![entity]).await?;

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

    pub async fn insert_entry(
        &mut self,
        table_entry: bfrt::bfrt::TableEntry,
    ) -> Result<(), ClientBasicError> {
        let client: &mut Client = self.client.borrow_mut();

        let update = bfrt::bfrt::Update {
            r#type: bfrt::bfrt::update::Type::Insert as i32,
            entity: Some(bfrt::bfrt::Entity {
                entity: Some(bfrt::bfrt::entity::Entity::TableEntry(table_entry)),
            }),
        };

        client.write(vec![update]).await?;

        Ok(())
    }

    pub async fn modify_entry(
        &mut self,
        table_entry: bfrt::bfrt::TableEntry,
    ) -> Result<(), ClientBasicError> {
        let client: &mut Client = self.client.borrow_mut();

        let update = bfrt::bfrt::Update {
            r#type: bfrt::bfrt::update::Type::Modify as i32,
            entity: Some(bfrt::bfrt::Entity {
                entity: Some(bfrt::bfrt::entity::Entity::TableEntry(table_entry)),
            }),
        };

        client.write(vec![update]).await?;

        Ok(())
    }

    pub async fn delete_entry(
        &mut self,
        table_entry: bfrt::bfrt::TableEntry,
    ) -> Result<(), ClientBasicError> {
        let client: &mut Client = self.client.borrow_mut();

        let update = bfrt::bfrt::Update {
            r#type: bfrt::bfrt::update::Type::Delete as i32,
            entity: Some(bfrt::bfrt::Entity {
                entity: Some(bfrt::bfrt::entity::Entity::TableEntry(table_entry)),
            }),
        };

        client.write(vec![update]).await?;

        Ok(())
    }
}
