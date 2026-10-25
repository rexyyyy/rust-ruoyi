use crate::modules::system::domain::{SysTrash, SysTrashActiveModel};
use crate::pool;
use serde::Serialize;
use crate::error::Error;

use sea_orm::entity::*;
use sea_orm::DatabaseTransaction;
use crate::modules::system::dao::*;

/// A trash can service that can recycle data. Retrieve the data, display the trash can data
pub struct SysTrashService {}

impl SysTrashService {
    /// 把待删除记录序列化成回收站行。
    fn build_rows<T>(table_name: &str, args: &[T]) -> Vec<SysTrashActiveModel>
    where
        T: Serialize,
    {
        let mut trashes = Vec::with_capacity(args.len());
        for x in args {
            trashes.push(
                SysTrash {
                    id: crate::util::object_id::new_object_id(),
                    table_name: Some(table_name.to_string()),
                    data: Some(serde_json::to_string(x).unwrap_or_default()),
                    create_time: Some(chrono::Local::now().naive_local()),
                }
                .into_active_model(),
            );
        }
        trashes
    }

    /// 写入回收站（独立连接，自带一次自治写入）。
    ///
    /// 适用于本来就是单步操作的场景。**若与业务删除同属一个原子操作，
    /// 请改用 [`Self::add_in_txn`]**，否则删除成功而回收失败会丢快照。
    pub async fn add<T>(&self, table_name: &str, args: &[T]) -> Result<u64, Error>
    where
        T: Serialize,
    {
        if args.is_empty() {
            return Ok(0);
        }
        Ok(insert_batch(pool!(), Self::build_rows(table_name, args)).await?)
    }

    /// 在**已开启的事务**内写入回收站。
    ///
    /// 事务语义由调用方掌控：成功时 `commit()`；任一步失败时用 `?` 提前返回，
    /// `DatabaseTransaction` 被 drop 会自动 rollback，从而保证
    /// 「删除业务记录」与「写入回收站」要么都发生、要么都不发生。
    pub async fn add_in_txn<T>(
        &self,
        txn: &DatabaseTransaction,
        table_name: &str,
        args: &[T],
    ) -> Result<u64, Error>
    where
        T: Serialize,
    {
        if args.is_empty() {
            return Ok(0);
        }
        Ok(insert_batch(txn, Self::build_rows(table_name, args)).await?)
    }
}
