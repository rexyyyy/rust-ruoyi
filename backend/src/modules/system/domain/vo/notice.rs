use crate::modules::system::domain::{SysNotice};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysNoticeVO {
    pub notice_id: Option<String>,
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_type: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

impl From<SysNotice> for SysNoticeVO {
    fn from(arg: SysNotice) -> Self {
        Self {
            notice_id: Some(arg.notice_id.to_string()),
            notice_title: arg.notice_title,
            notice_content: arg.notice_content,
            notice_type: arg.notice_type,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
            status: arg.status,
            remark: arg.remark,
        }
    }
}
