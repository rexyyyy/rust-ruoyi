use crate::modules::system::domain::SysNotice;
use crate::modules::system::domain::PageRequest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub notice_title: Option<String>,
    pub create_by: Option<String>,
    pub notice_type: Option<String>,
}

impl From<NoticePageDTO> for PageRequest {
    fn from(arg: NoticePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&NoticePageDTO> for PageRequest {
    fn from(arg: &NoticePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticeAddDTO {
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_type: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

impl From<NoticeAddDTO> for SysNotice {
    fn from(arg: NoticeAddDTO) -> Self {
        SysNotice {
            notice_id: crate::util::idseq::next_id() as i64,
            notice_title: arg.notice_title,
            notice_content: arg.notice_content,
            notice_type: arg.notice_type,
            status: arg.status,
            create_by: None,
            create_time: chrono::Local::now().naive_local() .into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticeUpdateDTO {
    pub notice_id: Option<String>,
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_type: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

impl From<NoticeUpdateDTO> for SysNotice {
    fn from(arg: NoticeUpdateDTO) -> Self {
        SysNotice {
            notice_id: arg.notice_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            notice_title: arg.notice_title,
            notice_content: arg.notice_content,
            notice_type: arg.notice_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: chrono::Local::now().naive_local() .into(),
            remark: arg.remark,
        }
    }
}
