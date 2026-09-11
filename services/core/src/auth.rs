use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

const JWT_ISSUER: &str = "olympus-core";
const JWT_AUDIENCE: &str = "olympus-api";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iss: String,
    pub aud: String,
}

fn decode_token(token: &str, secret: &str) -> Result<Claims, StatusCode> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.set_issuer(&[JWT_ISSUER]);
    validation.set_audience(&[JWT_AUDIENCE]);

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
        .filter(|token| !token.is_empty() && !token.chars().any(char::is_whitespace))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut claims = decode_token(token, &state.jwt_secret)?;

    let user: Option<(String, String, String)> = sqlx::query_as(
        "SELECT email, role, status FROM users WHERE id = $1",
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (email, role, status) = user.ok_or(StatusCode::UNAUTHORIZED)?;
    if status != "ACTIVE" {
        return Err(StatusCode::FORBIDDEN);
    }

    claims.email = email;
    claims.role = role;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::{decode_token, Claims, JWT_AUDIENCE, JWT_ISSUER};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use uuid::Uuid;

    fn claims(exp: usize) -> Claims {
        Claims {
            sub: Uuid::new_v4(),
            email: "user@example.com".into(),
            role: "BUYER".into(),
            exp,
            iss: JWT_ISSUER.into(),
            aud: JWT_AUDIENCE.into(),
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
        assert_eq!(decoded.iss, JWT_ISSUER);
        assert_eq!(decoded.aud, JWT_AUDIENCE);
    }

    #[test]
    fn rejects_wrong_issuer() {
        let secret = "test-secret";
        let mut expected = claims((chrono::Utc::now().timestamp() + 300) as usize);
        expected.iss = "another-service".into();
        let token = encode(&Header::default(), &expected, &EncodingKey::from_secret(secret.as_bytes())).unwrap();
        assert_eq!(decode_token(&token, secret), Err(axum::http::StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn rejects_wrong_audience() {
        let secret = "test-secret";
        let mut expected = claims((chrono::Utc::now().timestamp() + 300) as usize);
        expected.aud = "another-api".into();
        let token = encode(&Header::default(), &expected, &EncodingKey::from_secret(secret.as_bytes())).unwrap();
        assert_eq!(decode_token(&token, secret), Err(axum::http::StatusCode::UNAUTHORIZED));
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
        assert_eq!(decode_token(&token, "wrong-secret"), Err(axum::http::StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn rejects_malformed_token() {
        assert_eq!(decode_token("not-a-jwt", "test-secret"), Err(axum::http::StatusCode::UNAUTHORIZED));
    }
}
