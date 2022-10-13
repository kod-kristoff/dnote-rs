pub struct Book {}

impl Book {
    pub fn new() -> Self {
        log::trace!("creating Book ()");
        Self {}
    }
}
