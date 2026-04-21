use thiserror::Error;

/// Error type returned by every fallible operation in the SDK.
#[derive(Debug, Error)]
pub enum MadeOnSolError {
    /// API key was missing or malformed at construction time.
    ///
    /// Get a free key (200 req/day, no card) at <https://madeonsol.com/developer>.
    #[error(
        "MadeOnSol: apiKey is required and must start with `msk_`. \
         Get a free key at https://madeonsol.com/developer"
    )]
    MissingApiKey,

    /// API returned a non-2xx HTTP status.
    #[error("MadeOnSol API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        body: serde_json::Value,
    },

    /// `reqwest` transport error (DNS, TLS, connection reset, etc).
    #[error("MadeOnSol transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON serialization or deserialization error.
    #[error("MadeOnSol JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL-building error (only fires for impossible parameter combinations).
    #[error("MadeOnSol URL error: {0}")]
    Url(#[from] url::ParseError),
}

impl MadeOnSolError {
    /// Returns the HTTP status code if this is an [`MadeOnSolError::Api`] variant.
    pub fn status(&self) -> Option<u16> {
        match self {
            MadeOnSolError::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Returns the raw response body if this is an [`MadeOnSolError::Api`] variant.
    pub fn body(&self) -> Option<&serde_json::Value> {
        match self {
            MadeOnSolError::Api { body, .. } => Some(body),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, MadeOnSolError>;
