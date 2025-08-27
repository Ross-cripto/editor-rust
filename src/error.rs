use std::io;

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e.kind())
    }
}


