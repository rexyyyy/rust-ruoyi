
use crate::modules::system::domain::bo::notice::NoticePageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysNotice, SysNoticeEntity, SysNoticeColumn,
};
use crate::modules::system::domain::vo::SysNoticeVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::modules::system::dao::*;
/// notice service
pub struct SysNoticeService {}

impl SysNoticeService {
    pub async fn page(&self, arg: &NoticePageDTO) -> Result<Page<SysNoticeVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysNoticeEntity::find();
        if let Some(title) = &arg.notice_title {
            if !title.is_empty() {
                select = select.filter(SysNoticeColumn::NoticeTitle.contains(title.as_str()));
            }
        }
        if let Some(by) = &arg.create_by {
            if !by.is_empty() {
                select = select.filter(SysNoticeColumn::CreateBy.eq(by.clone()));
            }
        }
        if let Some(nt) = &arg.notice_type {
            if !nt.is_empty() {
                select = select.filter(SysNoticeColumn::NoticeType.eq(nt.to_string()));
            }
        }
        select = select.order_by_asc(SysNoticeColumn::NoticeTitle);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysNoticeVO>())
    }

    pub async fn finds_all(&self) -> Result<Vec<SysNoticeVO>> {
        let data = SysNoticeEntity::find().all(pool!()).await?;
        let mut notice_vos = vec![];
        for s in data {
            notice_vos.push(SysNoticeVO::from(s));
        }
        Ok(notice_vos)
    }
    pub async fn detail(&self, notice_id: &str) -> Result<SysNoticeVO> {
        // notice_id 在库中为 bigint，按字符串比较会导致 PostgreSQL 类型不匹配而查不到数据
        let notice = SysNoticeEntity::find()
            .filter(SysNoticeColumn::NoticeId.eq(notice_id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", notice_id)))?;
        let notice_vo = SysNoticeVO::from(notice);
        return Ok(notice_vo);
    }

    pub async fn add(&self, arg: &SysNotice) -> Result<u64> {
        let result = insert_model(pool!(), arg.clone()).await;
        return result;
    }

    pub async fn update(&self, data: SysNotice) -> Result<u64> {
        let old = SysNoticeEntity::find()
            .filter(SysNoticeColumn::NoticeId.eq(data.notice_id.clone()))
            .one(pool!())
            .await?;
        let mut merged = match old {
            None => { return Err(Error::from("参数错误")); }
            Some(c) => c,
        };
        // 与原 rbatis update_by_map 跳过 null 的语义一致：仅覆盖 DTO 中提供的字段
        merged.notice_title = data.notice_title.or(merged.notice_title);
        merged.notice_content = data.notice_content.or(merged.notice_content);
        merged.notice_type = data.notice_type.or(merged.notice_type);
        merged.status = data.status.or(merged.status);
        merged.update_by = data.update_by.or(merged.update_by);
        merged.update_time = data.update_time.or(merged.update_time);
        merged.remark = data.remark.or(merged.remark);
        let result = update_model(pool!(), merged).await;
        return Ok(result?);
    }

    pub async fn remove(&self, notice_id: &str) -> Result<u64> {
        use sea_orm::TransactionTrait;
        let notice_id_i64 = notice_id.parse::<i64>().unwrap_or(0);

        // 「删通知 + 写回收站」必须原子；快照在事务内读取以保证与删除一致。
        let txn = pool!().begin().await.map_err(Error::from)?;

        let targets = SysNoticeEntity::find()
            .filter(SysNoticeColumn::NoticeId.eq(notice_id_i64))
            .all(&txn)
            .await?;

        let r = SysNoticeEntity::delete_many()
            .filter(SysNoticeColumn::NoticeId.eq(notice_id_i64))
            .exec(&txn)
            .await?
            .rows_affected;
        if r > 0 {
            //copy data to trash
            CONTEXT
                .sys_trash_service
                .add_in_txn(&txn, "sys_notice", &targets)
                .await?;
        }

        txn.commit().await.map_err(Error::from)?;
        Ok(r)
    }
}
