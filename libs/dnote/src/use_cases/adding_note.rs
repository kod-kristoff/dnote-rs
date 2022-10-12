use crate::value_objects::BookName;

pub struct AddingNote {}

impl AddingNote {
    pub fn new() -> Self {
        log::trace!("creating use case AddingNote");
        Self {}
    }

    pub fn execute(&self, book: BookName, content: String) {
        log::trace!("called AddingNote::execute");
    }
}
