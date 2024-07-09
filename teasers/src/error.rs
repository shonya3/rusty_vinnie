use crate::ParseTeasersThreadError;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ParseTeasersThread(ParseTeasersThreadError),
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
