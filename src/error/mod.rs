use std::{result, error, io, boxed};
use core::{convert, fmt};
use serde::{ser, de};

pub struct Error {
    err: boxed::Box<ErrorImpl>,
}

impl Error {
    pub fn line(&self) -> usize { self.err.line() }
    pub fn column(&self) -> usize { self.err.colmn() }
    pub fn classify(&self) -> Category { self.err.classify() }
    pub fn is_io(&self) -> bool { self.classify() == Category::Io }
    pub fn is_syntax(&self) -> bool { self.classify() == Category::Syntax }
    pub fn is_data(&self) -> bool { self.classify() == Category::Data }
    pub fn is_eof(&self) -> bool { self.classify() == Category::Eof }
}

impl convert::From<Error> for io::Error {
    fn from(j: Error) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        unimplemented!()
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}

pub type Result<T> = result::Result<T, Error>;

struct ErrorImpl {
    line: usize,
    colmn: usize,
    category: Category,
}

impl  ErrorImpl {
    fn line(&self) -> usize { self.line }
    fn colmn(&self) -> usize { self.colmn }
    fn classify(&self) -> Category { self.category }
}