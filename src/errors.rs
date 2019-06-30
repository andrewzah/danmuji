use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, AppError>;

pub struct AppError(Box<ErrorKind>);

impl AppError {
    pub(crate) fn new(kind: ErrorKind) -> AppError {
        AppError(Box::new(kind))
    }

    pub(crate) fn from_string(msg: &str) -> AppError {
        AppError(Box::new(ErrorKind::Generic(msg.into())))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }

    //pub fn is_io_error(&self) -> bool {
        //match *self.0 {
            //ErrorKind::Io(_) => true,
            //_ => false,
        //}
    //}
}

#[derive(Debug)]
pub enum ErrorKind {
    Generic(String),
    R2D2(r2d2::Error),
    DbResult(diesel::result::Error),
    Regex(regex::Error)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Generic(ref msg) => {
                write!(f, "Generic error: {}", msg)
            }
            ErrorKind::R2D2(ref err) => err.fmt(f),
            ErrorKind::DbResult(ref err) => err.fmt(f),
            ErrorKind::Regex(ref err) => err.fmt(f),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> AppError {
        AppError::new(ErrorKind::R2D2(err))
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> AppError {
        AppError::new(ErrorKind::DbResult(err))
    }
}
