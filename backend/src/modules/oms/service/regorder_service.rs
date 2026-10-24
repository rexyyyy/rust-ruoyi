//! OmsRegorder 服务层
//! 源表：oms_regorder　业务：注册码登记
//! 由 codegen 于 2026-10-10 16:24:42 生成。
//!
//! 数据访问统一走 `system::dao` 的助手函数（`paginate` / `insert_model` / `update_model`），
//! 与 backend 既有模块保持同一套写法。

use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;

use crate::error::Error;
use crate::error::Result;
use crate::modules::system::dao::{insert_model, paginate, update_model};
use crate::modules::system::domain::{Page, PageRequest};
use crate::modules::system::service::CONTEXT;
use crate::pool;

use crate::modules::oms::domain::bo::{
    OmsRegorderAddDTO, OmsRegorderPageDTO, OmsRegorderUpdateDTO,
};
use crate::modules::oms::domain::entity::{
    OmsRegorder, OmsRegorderColumn, OmsRegorderEntity,
};
use crate::modules::oms::domain::vo::OmsRegorderVO;

pub struct OmsRegorderService {}

impl OmsRegorderService {
    /// 分页列表
    pub async fn page(&self, arg: &OmsRegorderPageDTO) -> Result<Page<OmsRegorderVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = OmsRegorderEntity::find();

        if let Some(v) = &arg.member_username {
            if !v.is_empty() {
                select = select.filter(OmsRegorderColumn::MemberUsername.contains(v.as_str()));
            }
        }

        if let Some(v) = &arg.pay_type {
            if !v.is_empty() {
                select = select.filter(OmsRegorderColumn::PayType.contains(v.as_str()));
            }
        }

        if let Some(v) = &arg.source_type {
            if !v.is_empty() {
                select = select.filter(OmsRegorderColumn::SourceType.contains(v.as_str()));
            }
        }

        if let Some(v) = &arg.status {
            if !v.is_empty() {
                select = select.filter(OmsRegorderColumn::Status.contains(v.as_str()));
            }
        }

        if let Some(v) = &arg.order_type {
            if !v.is_empty() {
                select = select.filter(OmsRegorderColumn::OrderType.contains(v.as_str()));
            }
        }

        if let Some(v) = &arg.receiver_name {
            if !v.is_empty() {
                select = select.filter(OmsRegorderColumn::ReceiverName.contains(v.as_str()));
            }
        }

        select = select.order_by_desc(OmsRegorderColumn::Regid);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<OmsRegorderVO>())
    }

    /// 全部（不分页）
    pub async fn finds_all(&self) -> Result<Vec<OmsRegorderVO>> {
        let data = OmsRegorderEntity::find()
            .order_by_desc(OmsRegorderColumn::Regid)
            .all(pool!())
            .await?;
        Ok(data.into_iter().map(OmsRegorderVO::from).collect())
    }

    /// 按主键取单条
    pub async fn detail(&self, id: &str) -> Result<OmsRegorderVO> {
        let row = OmsRegorderEntity::find()
            .filter(OmsRegorderColumn::Regid.eq(id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{}", id)))?;
        Ok(OmsRegorderVO::from(row))
    }

    /// 新增
    pub async fn add(&self, arg: &OmsRegorder) -> Result<u64> {
        Ok(insert_model(pool!(), arg.clone()).await?)
    }

    /// 修改。DTO 未提供到的字段保持原值（`Option::take().or()` 语义），
    /// 避免前端只提交部分表单时把其余列清空。
    pub async fn update(&self, data: OmsRegorder) -> Result<u64> {
        let old = OmsRegorderEntity::find()
            .filter(OmsRegorderColumn::Regid.eq(data.regid.clone()))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from("参数错误"))?;
        let mut merged = old;
        merged.order_sn = data.order_sn.or(merged.order_sn.take());
        merged.member_username = data.member_username.or(merged.member_username.take());
        merged.total_amount = data.total_amount.or(merged.total_amount.take());
        merged.pay_amount = data.pay_amount.or(merged.pay_amount.take());
        merged.pay_type = data.pay_type.or(merged.pay_type.take());
        merged.source_type = data.source_type.or(merged.source_type.take());
        merged.status = data.status.or(merged.status.take());
        merged.order_type = data.order_type.or(merged.order_type.take());
        merged.delivery_company = data.delivery_company.or(merged.delivery_company.take());
        merged.request_sn = data.request_sn.or(merged.request_sn.take());
        merged.delivery_sn = data.delivery_sn.or(merged.delivery_sn.take());
        merged.receiver_name = data.receiver_name.or(merged.receiver_name.take());
        merged.receiver_phone = data.receiver_phone.or(merged.receiver_phone.take());
        merged.receiver_province = data.receiver_province.or(merged.receiver_province.take());
        merged.receiver_city = data.receiver_city.or(merged.receiver_city.take());
        merged.receiver_region = data.receiver_region.or(merged.receiver_region.take());
        merged.note = data.note.or(merged.note.take());
        merged.confirm_status = data.confirm_status.or(merged.confirm_status.take());
        merged.payment_time = data.payment_time.or(merged.payment_time.take());
        merged.delivery_time = data.delivery_time.or(merged.delivery_time.take());
        merged.receive_time = data.receive_time.or(merged.receive_time.take());
        merged.comment_time = data.comment_time.or(merged.comment_time.take());
        merged.del_flag = data.del_flag.or(merged.del_flag.take());
        merged.create_by = data.create_by.or(merged.create_by.take());
        merged.create_time = data.create_time.or(merged.create_time.take());
        merged.update_by = data.update_by.or(merged.update_by.take());
        merged.update_time = data.update_time.or(merged.update_time.take());

        Ok(update_model(pool!(), merged).await?)
    }

    /// 删除（写回收站）
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let targets = OmsRegorderEntity::find()
            .filter(OmsRegorderColumn::Regid.eq(id.parse::<i64>().unwrap_or(0)))
            .all(pool!())
            .await?;
        let rows_affected = OmsRegorderEntity::delete_many()
            .filter(OmsRegorderColumn::Regid.eq(id.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?
            .rows_affected;
        if rows_affected > 0 {
            // copy data to trash
            CONTEXT.sys_trash_service.add("oms_regorder", &targets).await?;
        }
        Ok(rows_affected)
    }
}
