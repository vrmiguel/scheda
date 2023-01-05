use std::borrow::Cow;

#[derive(Debug)]
pub enum Error {
    MissingWhenStmt,
    MalformedRange(&'static str),
    UnknownDateTimePart(Box<str>),
    InvalidSyntax(Cow<'static, str>),
}

pub type Result<T> = std::result::Result<T, Error>;
