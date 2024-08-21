//! Module which contains all types of error for alpha vantage crates
use thiserror::Error;

/// Result type for alpha vantage crate
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Main error/failure enum
pub enum Error {
    /// Error which is raised if information is returned by API instead of data
    /// from API
    #[error("information: {0}")]
    AlphaVantageInformation(String),

    /// Error which is raised if error message is returned by API instead of
    /// data from API
    #[error("error_message: {0}")]
    AlphaVantageErrorMessage(String),

    /// Error which is raised if note is returned by API instead of data from
    /// API
    #[error("note: {0}")]
    AlphaVantageNote(String),

    /// Error which is raised if alpha vantage server returns some invalid data
    #[error("alpha vantage returns invalid data")]
    AlphaVantageInvalidData,

    /// Error which is raised when desired number of data is not present
    #[error("desired number of latest data not found try using less than {0} as n")]
    DesiredNumberOfDataNotPresent(usize),

    /// Error which is raised if API return empty response instead of returning
    /// data
    #[error("server returned empty response")]
    EmptyResponse,

    /// Error which is raise if failed to get output from server
    #[error("failed to get output from sever")]
    GetRequestFailed,

    /// Error which is raised if client fails to decode it into struct
    #[error("failed to decode string into struct")]
    DecodeJsonToStruct,

    /// Error which is raised if url is failed to get created
    #[error("failed to create url")]
    CreateUrl,
}

pub(crate) fn detect_common_helper_error(
    information: Option<String>,
    error_message: Option<String>,
    note: Option<String>,
) -> Result<()> {
    if let Some(information) = information {
        return Err(Error::AlphaVantageInformation(information));
    }
    if let Some(error_message) = error_message {
        return Err(Error::AlphaVantageErrorMessage(error_message));
    }
    if let Some(note) = note {
        return Err(Error::AlphaVantageNote(note));
    }
    Ok(())
}
