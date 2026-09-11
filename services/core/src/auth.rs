use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

fn decode_token(token: &str, secret: &str) -> Result<Claims, StatusCode> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|decoded| decoded.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let authorization = request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = decode_token(token, &state.jwt_secret)?;

    let status: Option<String> = sqlx::query_scalar("SELECT status FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match status.as_deref() {
        Some("ACTIVE") => {}
        Some(_) => return Err(StatusCode::FORBIDDEN),
        None => return Err(StatusCode::UNAUTHORIZED),
    }

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::{decode_token, Claims};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use uuid::Uuid;

    fn claims(exp: usize) -> Claims {
        Claims {
            sub: Uuid::new_v4(),
            email: "user@example.com".into(),
            role: "BUYER".into(),
            exp,
        }
    }

    #[test]
    fn accepts_valid_token() {
        let secret = "test-secret";
        let expected = claims((chrono::Utc::now().timestamp() + 300) as usize);
        let token = encode(
            &Header::default(),
            &expected,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let decoded = decode_token(&token, secret).unwrap();
        assert_eq!(decoded.sub, expected.sub);
        assert_eq!(decoded.email, expected.email);
        assert_eq!(decoded.role, expected.role);
    }

    #[test]
    fn rejects_expired_token() {
        let secret = "test-secret";
        let token = encode(
            &Header::default(),
            &claims((chrono::Utc::now().timestamp() - 1) as usize),
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        assert_eq!(decode_token(&token, secret), Err(axum::http::StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn rejects_wrong_secret() {
        let token = encode(
            &Header::default(),
            &claims((chrono::Utc::now().timestamp() + 300) as usize),
            &EncodingKey::from_secret(b"correct-secret"),
        )
        .unwrap();

        assert_eq!(
            decode_token(&token, "wrong-secret"),
            Err(axum::http::StatusCode::UNAUTHORIZED)
        );
    }

    #[test]
    fn rejects_malformed_token() {
        assert_eq!(
            decode_token("not-a-jwt", "test-secret"),
            Err(axum::http::StatusCode::UNAUTHORIZED)
        );
    }
}
