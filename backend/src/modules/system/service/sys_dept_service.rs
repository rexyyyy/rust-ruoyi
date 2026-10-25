use crate::modules::system::domain::bo::dept::{DeptQueryDTO};
use crate::modules::system::domain::{SysDept, SysDeptEntity, SysDeptColumn};

use crate::modules::system::domain::vo::{DeptTreeVO, SysDeptVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::TransactionTrait;
use crate::modules::system::dao::*;
//const DICT_KEY: &'static str = "sys_dept:all";

/// dictionary service
pub struct SysDeptService {}

impl SysDeptService {
    pub async fn all(&self, arg: &DeptQueryDTO) -> Result<Vec<SysDeptVO>> {
        let mut select = SysDeptEntity::find();
        select = select.filter(SysDeptColumn::ParentId.is_not_null());
        if let Some(name) = &arg.dept_name {
            if !name.is_empty() {
                select = select.filter(SysDeptColumn::DeptName.contains(name));
            }
        }
        if let Some(status) = &arg.status {
            select = select.filter(SysDeptColumn::Status.eq(status.to_string()));
        }
        select = select.order_by_asc(SysDeptColumn::OrderNum);
        let data = select.all(pool!()).await?;
        let mut res = vec![];
        for d in data {
            res.push(SysDeptVO::from(d));
        }
        Ok(res)
    }


    pub async fn detail(&self, dept_id: &str) -> Result<SysDeptVO> {
        // dept_id 在库中为 bigint，按字符串比较会导致 PostgreSQL 类型不匹配而查不到数据
        let dept_id_i64 = dept_id.parse::<i64>().unwrap_or(0);
        let dept = SysDeptEntity::find()
            .filter(SysDeptColumn::DeptId.eq(dept_id_i64))
            .all(pool!())
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dept_id)))?;
        let mut dept_vo = SysDeptVO::from(dept);
        // 补充上级部门名称，供前端部门编辑表单回显 parentName
        if let Some(pid) = dept_vo.parent_id.clone() {
            if !pid.is_empty() && pid != "0" {
                let parent = SysDeptEntity::find()
                    .filter(SysDeptColumn::DeptId.eq(pid.parse::<i64>().unwrap_or(0)))
                    .one(pool!())
                    .await?;
                dept_vo.parent_name = parent.and_then(|d| d.dept_name);
            }
        }
        return Ok(dept_vo);
    }

    pub async fn add(&self, arg: &SysDept) -> Result<u64> {
        let result = insert_model(pool!(), arg.clone()).await?;
        return Ok(result);
    }

    pub async fn update(&self, data: SysDept) -> Result<u64> {
        let result = update_model(pool!(), data.clone()).await?;
        return Ok(result);
    }

    pub async fn remove(&self, dept_id: &str) -> Result<u64> {
        let dept_id_i64 = dept_id.parse::<i64>().unwrap_or(0);

        // 「删部门 + 写回收站」必须原子：否则删除成功却丢了快照，回收站将失去恢复能力。
        // 任一步 `?` 提前返回时 `txn` 被 drop，sea-orm 自动 rollback。
        let txn = pool!().begin().await.map_err(Error::from)?;

        // 快照在事务内读取，保证与删除操作看到一致的数据
        let targets = SysDeptEntity::find()
            .filter(SysDeptColumn::DeptId.eq(dept_id_i64))
            .all(&txn)
            .await?;

        let rows_affected = SysDeptEntity::delete_many()
            .filter(SysDeptColumn::DeptId.eq(dept_id_i64))
            .exec(&txn)
            .await?
            .rows_affected;
        if rows_affected > 0 {
            //copy data to trash
            CONTEXT
                .sys_trash_service
                .add_in_txn(&txn, "sys_dept", &targets)
                .await?;
        }

        txn.commit().await.map_err(Error::from)?;
        Ok(rows_affected)
    }
    //根据user id获得本单位及下属单位部门列表
    pub async fn get_dept_tree(&self, user_id: &str) -> Result<Vec<DeptTreeVO>> {
        let depts = SysDeptEntity::find()
            .filter(SysDeptColumn::ParentId.is_not_null())
            .order_by_asc(SysDeptColumn::OrderNum)
            .all(pool!())
            .await?;
        let mut res = vec![];
        for d in depts {
            res.push(DeptTreeVO::from(d));
        }
       self. build_dept_tree(&res)
    }
    ///An depts array with a hierarchy
    pub fn build_dept_tree(&self, all_depts: &Vec<DeptTreeVO>) -> Result<Vec<DeptTreeVO>> {
        //find tops
        let mut tops = vec![];
        for item in all_depts {
            //parent id null, it is an top menu
            if item.is_parent() {
                tops.push(item.clone());
            }
        }
        //find child
      //  tops.sort_by(|a, b| a.order_num.cmp(&b.order_num));
        for mut item in &mut tops {
            self.loop_find_children(&mut item, &all_depts);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_children(&self, arg: &mut DeptTreeVO, all_depts: &Vec<DeptTreeVO>) {
        let mut children = vec![];
        for item in all_depts {
            if !item.is_parent() && item.parent_id == arg.id {
                let mut item = item.clone();
                self.loop_find_children(&mut item, all_depts);
                children.push(item);
            }
        }
        if !children.is_empty() {
         //   children.sort_by(|a, b| a.order_num.cmp(&b.order_num));
            arg.children = Some(children);
        }
    }

}
