//! PaginatedVec model

use models::api::pagination::Pagination;

pub struct PaginatedVec<T> {
    pub vec: Vec<T>,
    pub pag: Option<Pagination>,
}
