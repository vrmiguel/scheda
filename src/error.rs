#[derive(Debug)]
pub enum Error {
    MalformedRange(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
