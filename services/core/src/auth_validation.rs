pub fn normalize_email(email: &str) -> Option<String> {
    let email = email.trim().to_lowercase();
    if email.len() > 254 {
        return None;
    }
    let (local, domain) = email.split_once('@')?;
    if local.is_empty() || local.len() > 64 || domain.is_empty() || domain.len() > 253 || !domain.contains('.') {
        return None;
    }
    Some(email)
}

pub fn valid_registration(password: &str, full_name: &str) -> bool {
    let name = full_name.trim();
    password.len() >= 8
        && password.len() <= 128
        && !name.is_empty()
        && name.len() <= 160
}

pub fn valid_login_password(password: &str) -> bool {
    !password.is_empty() && password.len() <= 128
}

#[cfg(test)]
mod tests {
    use super::{normalize_email, valid_login_password, valid_registration};

    #[test]
    fn normalizes_valid_email() {
        assert_eq!(normalize_email("  USER@Example.COM "), Some("user@example.com".into()));
    }

    #[test]
    fn rejects_malformed_email() {
        assert_eq!(normalize_email("user"), None);
        assert_eq!(normalize_email("@example.com"), None);
        assert_eq!(normalize_email("user@example"), None);
        assert_eq!(normalize_email(&"a".repeat(255)), None);
    }

    #[test]
    fn validates_registration_fields() {
        assert!(valid_registration("password123", "Alice Example"));
        assert!(!valid_registration("short", "Alice"));
        assert!(!valid_registration(&"p".repeat(129), "Alice"));
        assert!(!valid_registration("password123", "   "));
        assert!(!valid_registration("password123", &"A".repeat(161)));
    }

    #[test]
    fn validates_login_password_size() {
        assert!(valid_login_password("password123"));
        assert!(!valid_login_password(""));
        assert!(!valid_login_password(&"p".repeat(129)));
    }
}
