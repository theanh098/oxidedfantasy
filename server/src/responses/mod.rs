use serde::Serialize;

pub mod auth;

#[derive(Serialize)]
pub struct PaginationResponse<T> {
    pub nodes: Vec<T>,
    pub page: u16,
    pub total: u64,
}
