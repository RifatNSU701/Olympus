use crate::auth::Claims;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    ManageProducts,
    SellerOperations,
    ViewSellerAnalytics,
    ManageUsers,
}

pub fn has_permission(role: &str, permission: Permission) -> bool {
    match permission {
        Permission::ManageProducts => matches!(role, "SELLER" | "ADMIN" | "SUPER_ADMIN"),
        Permission::SellerOperations => matches!(role, "SELLER" | "ADMIN" | "SUPER_ADMIN"),
        Permission::ViewSellerAnalytics => matches!(role, "SELLER" | "ADMIN" | "SUPER_ADMIN"),
        Permission::ManageUsers => matches!(role, "ADMIN" | "SUPER_ADMIN"),
    }
}

pub fn authorize(claims: &Claims, permission: Permission) -> Result<(), axum::http::StatusCode> {
    if has_permission(&claims.role, permission) {
        Ok(())
    } else {
        Err(axum::http::StatusCode::FORBIDDEN)
    }
}

#[cfg(test)]
mod tests {
    use super::{has_permission, Permission};

    #[test]
    fn seller_permissions_are_scoped() {
        assert!(has_permission("SELLER", Permission::ManageProducts));
        assert!(has_permission("SELLER", Permission::SellerOperations));
        assert!(has_permission("SELLER", Permission::ViewSellerAnalytics));
        assert!(!has_permission("SELLER", Permission::ManageUsers));
    }

    #[test]
    fn admin_permissions_include_seller_operations() {
        assert!(has_permission("ADMIN", Permission::ManageProducts));
        assert!(has_permission("ADMIN", Permission::SellerOperations));
        assert!(has_permission("ADMIN", Permission::ViewSellerAnalytics));
        assert!(has_permission("ADMIN", Permission::ManageUsers));
    }

    #[test]
    fn super_admin_has_all_permissions() {
        for permission in [
            Permission::ManageProducts,
            Permission::SellerOperations,
            Permission::ViewSellerAnalytics,
            Permission::ManageUsers,
        ] {
            assert!(has_permission("SUPER_ADMIN", permission));
        }
    }

    #[test]
    fn citizen_has_no_privileged_permissions() {
        for permission in [
            Permission::ManageProducts,
            Permission::SellerOperations,
            Permission::ViewSellerAnalytics,
            Permission::ManageUsers,
        ] {
            assert!(!has_permission("CITIZEN", permission));
        }
    }
}
