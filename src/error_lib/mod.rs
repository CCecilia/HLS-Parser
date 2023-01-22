use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PlaylistDownloadError {
    RequestFailed,
    FailedToReadResponse,
    FailedToCreateTempFile,
    FailedToCopyData,
}

impl Error for PlaylistDownloadError {}

impl fmt::Display for PlaylistDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlaylistDownloadError::RequestFailed => write!(f, "request for playlist failed"),
            PlaylistDownloadError::FailedToReadResponse => write!(f, "Failed to read request data"),
            PlaylistDownloadError::FailedToCreateTempFile => {
                write!(f, "Failed to create temp file")
            }
            PlaylistDownloadError::FailedToCopyData => {
                write!(f, "Failed to copy playlist to temp file")
            }
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    InvalidFile,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::InvalidFile => write!(f, "request for playlist failed"),
        }
    }
}
