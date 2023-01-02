#[derive(Debug)]
pub enum Error {
    MissingWhenStmt,
    MalformedRange(&'static str),
    UnknownDateTimePart(Box<str>),
    InvalidSyntax(Box<str>),
}

pub type Result<T> = std::result::Result<T, Error>;
