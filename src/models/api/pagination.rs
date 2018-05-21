//! Pagination related structs and implementations

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub offset : i64,
    pub limit : i64
}
