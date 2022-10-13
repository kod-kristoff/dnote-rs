use crate::{Book, BookName, Error, UniqueId};
pub trait BookRepo {
    fn get(&self, id: UniqueId) -> Result<Option<Book>, Error>;
    fn get_by_label(&self, label: &BookName) -> Result<Option<Book>, Error>;
}
