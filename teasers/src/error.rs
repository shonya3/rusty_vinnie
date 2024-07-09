use std::fmt::Display;

use crate::ParseTeasersThreadError;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ParseTeasersThread(ParseTeasersThreadError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReqwestError(err) => err.fmt(f),
            Error::ParseTeasersThread(err) => err.fmt(f),
        }
    }
}

impl From<ParseTeasersThreadError> for Error {
    fn from(value: ParseTeasersThreadError) -> Self {
        Self::ParseTeasersThread(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}
