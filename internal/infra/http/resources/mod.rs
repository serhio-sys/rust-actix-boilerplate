use serde::Serialize;

pub mod user_resouce;

#[derive(Serialize, Clone, PartialEq)]
pub struct BasedListResponse<T> where T: Serialize {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
}
