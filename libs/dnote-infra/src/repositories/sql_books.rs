pub struct SqlBookRepo {}

impl SqlBookRepo {
    pub fn new() -> Self {
        log::trace!("creating SqlBookRepo ...");
        Self {}
    }
}

impl dnote::BookRepo for SqlBookRepo {
    fn get(&self, id: dnote::UniqueId) -> Result<Option<dnote::Book>, dnote::Error> {
        log::trace!("get book with id '{}'", id);
        todo!("get book with id '{}'", id)
    }

    fn get_by_label(&self, label: &dnote::BookName) -> Result<Option<dnote::Book>, dnote::Error> {
        log::trace!("get book with label '{}'", label);
        todo!("get book with label '{}'", label)
    }
}
