use std::sync::Arc;

use crate::{repositories::books::BookRepo, value_objects::BookName, Error};

pub struct AddingNote {
    book_repo: Arc<dyn BookRepo>,
}

impl AddingNote {
    pub fn new(book_repo: Arc<dyn BookRepo>) -> Self {
        log::trace!("creating use case AddingNote");
        Self { book_repo }
    }

    pub fn execute(&self, book_name: &BookName, content: &String) -> Result<(), Error> {
        log::trace!("called AddingNote::execute");
        let book = self.book_repo.get_by_label(book_name)?;
        todo!("Add '{}' to book '{}'", content, book_name)
    }
}
