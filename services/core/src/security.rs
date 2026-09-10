use axum::{http::{header::HeaderName, HeaderValue, Request}, middleware::Next, response::Response};

pub async fn security_headers(request: Request<axum::body::Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-content-type-options", HeaderValue::from_static("nosniff"));
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    headers.insert("referrer-policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    headers.insert("content-security-policy", HeaderValue::from_static("default-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'"));
    headers.insert("permissions-policy", HeaderValue::from_static("camera=(), microphone=(), geolocation=()"));
    response
}

pub fn request_id_header() -> HeaderName {
    HeaderName::from_static("x-request-id")
}
