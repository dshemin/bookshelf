use serde::Serialize;

mod cursor;
pub mod storage;

pub use cursor::Cursor;

/// Max number of data per request to DB.
pub(crate) const LIMIT: usize = 25;

#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    data: Vec<T>,
    cursor: Option<Cursor>,
}
