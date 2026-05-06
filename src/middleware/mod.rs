use hyper::header::{HeaderName, HeaderValue};
use hyper::HeaderMap;

pub fn add_cors_headers(headers: &mut HeaderMap) {
    headers.insert(
        HeaderName::from_static("Access-Control-Allow-Origin"),
        HeaderValue::from_static("*"),
    );
    headers.insert(
        HeaderName::from_static("Access-Control-Allow-Methods"),
        HeaderValue::from_static("GET, POST, OPTIONS, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        HeaderName::from_static("Access-Control-Allow-Headers"),
        HeaderValue::from_static("Content-Type, Authorization"),
    );
}
