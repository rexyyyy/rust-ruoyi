use crate::modules::system::domain::SysPost;
use crate::modules::system::domain::PageRequest;
use serde::{Deserialize, Serialize};

/// page page DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub post_name: Option<String>,
    pub post_code: Option<String>,
    pub status: Option<String>,
}

impl From<PostPageDTO> for PageRequest {
    fn from(arg: PostPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&PostPageDTO> for PageRequest {
    fn from(arg: &PostPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostAddDTO {
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

impl From<PostAddDTO> for SysPost {
    fn from(arg: PostAddDTO) -> Self {
        SysPost {
            post_id: crate::util::idseq::next_id() as i64,
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort.map(|v| v as i32),
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
pub struct PostUpdateDTO {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

impl From<PostUpdateDTO> for SysPost {
    fn from(arg: PostUpdateDTO) -> Self {
        SysPost {
            post_id: arg.post_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort.map(|v| v as i32),
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: chrono::Local::now().naive_local() .into(),
            remark: arg.remark,
        }
    }
}
