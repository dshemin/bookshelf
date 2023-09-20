use serde::Serialize;

mod cursor;
pub mod storage;

pub use cursor::Cursor;

/// Max number of data per request to DB.
pub(crate) const LIMIT: usize = 25;

/// Represents paginated data.
#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    /// A small set of requested data.
    data: Vec<T>,
    /// Cursor for fetching next page.
    /// Will be None, if there is no any page left.
    cursor: Option<Cursor>,
}
