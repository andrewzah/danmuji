use std::{fmt, result};

use serenity::{http::Http, model::prelude::*};

pub type Result<T> = result::Result<T, AppError>;

pub struct AppError(Box<ErrorKind>);

// TODO: snafu to make this less verbose
impl AppError {
    pub(crate) fn new(kind: ErrorKind) -> AppError {
        AppError(Box::new(kind))
    }

    pub(crate) fn from_str(msg: &str) -> AppError {
        AppError(Box::new(ErrorKind::Generic(msg.into())))
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    #[allow(dead_code)]
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }

    //pub fn is_io_error(&self) -> bool {
    //match *self.0 {
    //ErrorKind::Io(_) => true,
    //_ => false,
    //}
}

#[derive(Debug)]
pub enum ErrorKind {
    Generic(String),
    R2D2(r2d2::Error),
    DbResult(diesel::result::Error),
    Regex(regex::Error),
    IO(std::io::Error),
    Serenity(serenity::Error),
    StrFmt(strfmt::FmtError),
    ParseInt(std::num::ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Generic(ref msg) => write!(f, "Generic error: {}", msg),
            ErrorKind::R2D2(ref err) => err.fmt(f),
            ErrorKind::DbResult(ref err) => err.fmt(f),
            ErrorKind::Regex(ref err) => err.fmt(f),
            ErrorKind::IO(ref err) => err.fmt(f),
            ErrorKind::Serenity(ref err) => err.fmt(f),
            ErrorKind::StrFmt(ref err) => err.fmt(f),
            ErrorKind::ParseInt(ref err) => err.fmt(f),
        }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Generic(ref msg) => write!(f, "Generic error: {}", msg),
            ErrorKind::R2D2(ref err) => err.fmt(f),
            ErrorKind::DbResult(ref err) => err.fmt(f),
            ErrorKind::Regex(ref err) => err.fmt(f),
            ErrorKind::IO(ref err) => err.fmt(f),
            ErrorKind::Serenity(ref err) => err.fmt(f),
            ErrorKind::StrFmt(ref err) => err.fmt(f),
            ErrorKind::ParseInt(ref err) => err.fmt(f),
        }
    }
}

impl AppError {
    pub fn send_err(&self, http: &Http, msg: &Message, why: String) -> Result<Message> {
        msg.channel_id
            .send_message(&http, |m| {
                m.embed(|e| {
                    e.title("Error");
                    e.description(format!(":x: {}: {}", why, *self));

                    e
                });

                m
            })
            .map_err(|err| AppError::new(ErrorKind::Serenity(err)))
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

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> AppError {
        AppError::new(ErrorKind::Regex(err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::new(ErrorKind::IO(err))
    }
}

impl From<serenity::Error> for AppError {
    fn from(err: serenity::Error) -> AppError {
        AppError::new(ErrorKind::Serenity(err))
    }
}

impl From<serenity::framework::standard::CommandError> for AppError {
    fn from(err: serenity::framework::standard::CommandError) -> AppError {
        AppError::new(ErrorKind::Generic(err.0))
    }
}

impl From<strfmt::FmtError> for AppError {
    fn from(err: strfmt::FmtError) -> AppError {
        AppError::new(ErrorKind::StrFmt(err))
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> AppError {
        AppError::new(ErrorKind::ParseInt(err))
    }
}
