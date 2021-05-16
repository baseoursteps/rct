pub type Result<T> = std::result::Result<T, Error>;

use std::io::Error as ioErr;

#[derive(Debug)]
pub enum Error {
    Io(ioErr),
    None,
    Misc(String),
}

impl From<ioErr> for Error {
    fn from(error: ioErr) -> Self {
        Error::Io(error)
    }
}
