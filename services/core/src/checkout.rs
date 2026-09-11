use rust_decimal::Decimal;

pub fn validate_checkout_inputs(
    idempotency_key: &str,
    shipping_address: &str,
    shipping: Decimal,
    tax: Decimal,
) -> bool {
    let key = idempotency_key.trim();
    let address = shipping_address.trim();
    !key.is_empty()
        && key.len() <= 120
        && !address.is_empty()
        && address.len() <= 500
        && !shipping.is_sign_negative()
        && !tax.is_sign_negative()
}

pub fn calculate_total(subtotal: Decimal, shipping: Decimal, tax: Decimal) -> Decimal {
    subtotal + shipping + tax
}

#[cfg(test)]
mod tests {
    use super::{calculate_total, validate_checkout_inputs};
    use rust_decimal::Decimal;

    #[test]
    fn validates_normal_checkout_values() {
        assert!(validate_checkout_inputs(
            "checkout-123",
            "Dhaka, Bangladesh",
            Decimal::new(1000, 2),
            Decimal::new(50, 2),
        ));
    }

    #[test]
    fn rejects_invalid_checkout_values() {
        assert!(!validate_checkout_inputs("", "Dhaka", Decimal::ZERO, Decimal::ZERO));
        assert!(!validate_checkout_inputs("key", "", Decimal::ZERO, Decimal::ZERO));
        assert!(!validate_checkout_inputs("key", "Dhaka", Decimal::new(-1, 2), Decimal::ZERO));
        assert!(!validate_checkout_inputs("key", "Dhaka", Decimal::ZERO, Decimal::new(-1, 2)));
        assert!(!validate_checkout_inputs(&"x".repeat(121), "Dhaka", Decimal::ZERO, Decimal::ZERO));
        assert!(!validate_checkout_inputs("key", &"x".repeat(501), Decimal::ZERO, Decimal::ZERO));
    }

    #[test]
    fn calculates_checkout_total_exactly() {
        let subtotal = Decimal::new(12500, 2);
        let shipping = Decimal::new(1500, 2);
        let tax = Decimal::new(1875, 2);
        assert_eq!(calculate_total(subtotal, shipping, tax), Decimal::new(15875, 2));
    }
}
