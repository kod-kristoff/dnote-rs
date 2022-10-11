#[derive(Clone, Debug, PartialEq)]
pub struct BookName(String);
pub struct BookNameRef<'a>(&'a str);

impl BookName {
    pub fn try_new(name: String) -> Result<Self, InvalidBookName> {
        if name.is_empty() {
            return Err(InvalidBookName::Empty);
        } else if ["trash", "conflicts"].contains(&name.as_str()) {
            return Err(InvalidBookName::Reserved);
        } else if name.contains(" ") {
            return Err(InvalidBookName::HasSpace);
        } else if name.contains("\n") {
            return Err(InvalidBookName::MultiLine);
        }
        Ok(Self(name))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvalidBookName {
    Empty,
    HasSpace,
    MultiLine,
    Reserved,
}

impl std::fmt::Display for InvalidBookName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "The book name is empty"),
            Self::HasSpace => write!(f, "The book name cannot contain spaces"),
            Self::MultiLine => write!(f, "The book name cannot contain line breaks"),
            Self::Reserved => write!(f, "The book name is reserved"),
        }
    }
}

impl std::error::Error for InvalidBookName {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_fails() {
        let r = BookName::try_new("".into());

        assert_eq!(r, Err(InvalidBookName::Empty));
    }

    #[test]
    fn reserved_name_trash_fails() {
        let r = BookName::try_new("trash".into());

        assert_eq!(r, Err(InvalidBookName::Reserved));
    }

    #[test]
    fn reserved_name_conflicts_fails() {
        let r = BookName::try_new("conflicts".into());

        assert_eq!(r, Err(InvalidBookName::Reserved));
    }

    #[test]
    fn name_with_space_fails() {
        let r = BookName::try_new("has space".into());

        assert_eq!(r, Err(InvalidBookName::HasSpace));
    }

    #[test]
    fn name_with_newline_fails() {
        let r = BookName::try_new("hasspace\n".into());

        assert_eq!(r, Err(InvalidBookName::MultiLine));
    }

    #[test]
    fn valid_name_succeeds() {
        let book_name = BookName::try_new("valid".into()).unwrap();

        assert_eq!(book_name, BookName("valid".into()));
    }
}
