use crate::modules::system::domain::SysTrash;
use crate::pool;
use serde::Serialize;
use crate::error::Error;

use sea_orm::entity::*;
use crate::modules::system::dao::*;

/// A trash can service that can recycle data. Retrieve the data, display the trash can data
pub struct SysTrashService {}

impl SysTrashService {
    pub async fn add<T>(&self, table_name: &str, args: &[T]) -> Result<u64, Error>
    where
        T: Serialize,
    {
        if args.is_empty() {
            return Ok(0);
        }
        //copy data to trash
        let mut trashes = Vec::with_capacity(args.len());
        for x in args {
            trashes.push(SysTrash {
                id: crate::util::object_id::new_object_id(),
                table_name: Some(table_name.to_string()),
                data: Some(serde_json::to_string(x).unwrap_or_default()),
                create_time: Some(chrono::Local::now().naive_local()),
            });
        }
        Ok(insert_batch(
            pool!(),
            trashes
                .iter()
                .cloned()
                .map(|m| m.into_active_model())
                .collect(),
        )
        .await?)
    }
}
