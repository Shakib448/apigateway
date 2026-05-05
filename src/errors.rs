use std::fmt;

#[derive(Debug)]
pub enum GatewayError {
    InvalidUri(String),
    Http(String),
    RateLimitExceeded,
    Timeout,
    Unauthorized,
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GatewayError::InvalidUri(s) => write!(f, "Invalid URI: {}", s),
            GatewayError::Http(s) => write!(f, "HTTP error: {}", s),
            GatewayError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            GatewayError::Timeout => write!(f, "Request timed out"),
            GatewayError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl warp::reject::Reject for GatewayError {}
