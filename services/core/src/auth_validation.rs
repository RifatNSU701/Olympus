pub fn normalize_email(email: &str) -> Option<String> {
    let email = email.trim().to_lowercase();
    let (local, domain) = email.split_once('@')?;
    if local.is_empty() || domain.is_empty() || !domain.contains('.') {
        return None;
    }
    Some(email)
}

pub fn valid_registration(password: &str, full_name: &str) -> bool {
    let name = full_name.trim();
    password.len() >= 8 && !name.is_empty() && name.len() <= 160
}

#[cfg(test)]
mod tests {
    use super::{normalize_email, valid_registration};

    #[test]
    fn normalizes_valid_email() {
        assert_eq!(normalize_email("  USER@Example.COM "), Some("user@example.com".into()));
    }

    #[test]
    fn rejects_malformed_email() {
        assert_eq!(normalize_email("user"), None);
        assert_eq!(normalize_email("@example.com"), None);
        assert_eq!(normalize_email("user@example"), None);
    }

    #[test]
    fn validates_registration_fields() {
        assert!(valid_registration("password123", "Alice Example"));
        assert!(!valid_registration("short", "Alice"));
        assert!(!valid_registration("password123", "   "));
        assert!(!valid_registration("password123", &"A".repeat(161)));
    }
}
