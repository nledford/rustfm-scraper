use thiserror::Error;

use crate::models::ErrorResponse;

#[derive(Clone, Debug, Error)]
pub enum LastFmApiError {
    #[error("{0}")]
    InvalidService(String),
    #[error("{0}")]
    InvalidMethod(String),
    #[error("{0}")]
    AuthenticationFailed(String),
    #[error("{0}")]
    InvalidFormat(String),
    #[error("{0}")]
    InvalidParameters(String),
    #[error("Invalid resource Specified")]
    InvalidResourceSpecified,
    #[error("{0}")]
    OperationFailed(String),
    #[error("{0}")]
    InvalidSessionKey(String),
    #[error("{0}")]
    InvalidApiKey(String),
    #[error("{0}")]
    ServiceOffline(String),
    #[error("Invalid method signature supplied")]
    InvalidMethodSignatureSupplied,
    #[error("{0}")]
    TemporaryError(String),
    #[error("{0}")]
    SuspendedApiKey(String),
    #[error("{0}")]
    RateLimitExceeded(String),
}

pub fn get_lastfm_api_error(error: ErrorResponse) -> LastFmApiError {
    match error.error {
        2 => LastFmApiError::InvalidService(error.message),
        3 => LastFmApiError::InvalidMethod(error.message),
        4 => LastFmApiError::AuthenticationFailed(error.message),
        5 => LastFmApiError::InvalidFormat(error.message),
        6 => LastFmApiError::InvalidParameters(error.message),
        7 => LastFmApiError::InvalidResourceSpecified,
        9 => LastFmApiError::InvalidSessionKey(error.message),
        10 => LastFmApiError::InvalidApiKey(error.message),
        11 => LastFmApiError::ServiceOffline(error.message),
        13 => LastFmApiError::InvalidMethodSignatureSupplied,
        16 => LastFmApiError::TemporaryError(error.message),
        26 => LastFmApiError::SuspendedApiKey(error.message),
        29 => LastFmApiError::RateLimitExceeded(error.message),

        8 | _ => LastFmApiError::OperationFailed(error.message),
    }
}