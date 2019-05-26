#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Io(std::io::Error),
    Rocks(rocksdb::Error),
    SerdeCbor(serde_cbor::error::Error),
}
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl From<rocksdb::Error> for Error {
    fn from(err: rocksdb::Error) -> Self {
        Error::Rocks(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<serde_cbor::error::Error> for Error {
    fn from(err: serde_cbor::error::Error) -> Self {
        Error::SerdeCbor(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;