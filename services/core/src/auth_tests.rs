#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use super::super::auth::Claims;
    use uuid::Uuid;

    #[test]
    fn valid_claims_decode() {
        let secret = "test-secret";
        let claims = Claims { sub: Uuid::new_v4(), email: "buyer@example.com".into(), role: "BUYER".into(), exp: (chrono::Utc::now().timestamp() + 3600) as usize };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())).unwrap();
        let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256)).unwrap();
        assert_eq!(decoded.claims.email, "buyer@example.com");
        assert_eq!(decoded.claims.role, "BUYER");
    }

    #[test]
    fn expired_token_is_rejected() {
        let secret = "test-secret";
        let claims = Claims { sub: Uuid::new_v4(), email: "buyer@example.com".into(), role: "BUYER".into(), exp: 1 };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())).unwrap();
        assert!(decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256)).is_err());
    }
}
