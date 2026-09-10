use axum::{extract::Request, http::{header, HeaderValue, StatusCode}, middleware::Next, response::Response};

pub async fn security_headers(request: Request, next: Next) -> Result<Response, StatusCode> {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("strict-origin-when-cross-origin"));
    headers.insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'"));
    headers.insert("permissions-policy", HeaderValue::from_static("camera=(), microphone=(), geolocation=()"));
    Ok(response)
}
