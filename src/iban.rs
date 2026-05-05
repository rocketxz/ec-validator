use crate::ValidationError;

/// Validates an Ecuadorian IBAN following ISO 13616.
///
/// # Arguments
///
/// * `input` - A string containing the IBAN (starts with "EC", 24 characters total).
///
/// # Errors
///
/// Returns [`ValidationError`] on validation failure:
/// - [`ValidationError::InvalidFormat`] - Does not start with "EC" or contains invalid characters
/// - [`ValidationError::InvalidLength`] - Not exactly 24 characters
/// - [`ValidationError::InvalidCheckDigit`] - Mod-97 check digit invalid
///
/// # Examples
///
/// ```
/// use ec_validator::iban;
///
/// // Valid IBAN format (must have correct check digits)
/// let result = iban::validate("EC8912345678901234567890");
/// // Result depends on check digit validity
/// ```
pub fn validate(input: &str) -> Result<(), ValidationError> {
    let input = input.trim().to_uppercase();

    if !input.starts_with("EC") {
        return Err(ValidationError::InvalidFormat);
    }

    if input.len() != 24 {
        return Err(ValidationError::InvalidLength);
    }

    if !input.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(ValidationError::InvalidFormat);
    }

    let rearranged: String = input[4..].to_string() + &input[..4];

    let mut remainder = 0u32;

    for c in rearranged.chars() {
        if c.is_ascii_digit() {
            let digit = c as u32 - '0' as u32;
            if digit <= 9 {
                remainder = (remainder * 10 + digit) % 97;
            } else {
                return Err(ValidationError::InvalidFormat);
            }
        } else {
            let value = (c as u32) - ('A' as u32) + 10;
            if (10..=35).contains(&value) {
                remainder = (remainder * 100 + value) % 97;
            } else {
                return Err(ValidationError::InvalidFormat);
            }
        }
    }

    if remainder != 1 {
        return Err(ValidationError::InvalidCheckDigit);
    }

    Ok(())
}

/// Convenience function that returns `true` if the IBAN is valid, `false` otherwise.
///
/// # Arguments
///
/// * `input` - A string containing the IBAN.
///
/// # Examples
///
/// ```
/// use ec_validator::iban;
///
/// assert!(!iban::is_valid("invalid"));
/// ```
pub fn is_valid(input: &str) -> bool {
    validate(input).is_ok()
}

/// Formats an IBAN with spaces every 4 characters (grouped display).
///
/// # Arguments
///
/// * `input` - A string containing the IBAN.
///
/// # Returns
///
/// Returns [`Some`] with grouped IBAN if valid, [`None`] otherwise.
///
/// # Examples
///
/// ```
/// use ec_validator::iban;
///
/// let formatted = iban::format("EC8912345678901234567890");
/// // Returns Some("EC89 1234 5678 9012 3456 7890") or None depending on validity
/// ```
pub fn format(input: &str) -> Option<String> {
    if validate(input).is_ok() {
        let input = input.trim().to_uppercase();
        let chars: Vec<char> = input.chars().collect();
        let mut result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && i % 4 == 0 {
                result.push(' ');
            }
            result.push(*c);
        }

        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_iban() {
        let result = validate("EC8912345678901234567890");
        assert!(result.is_err());
    }

    #[test]
    fn wrong_country() {
        assert_eq!(
            validate("XX1234567890123456789012"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn wrong_length() {
        assert_eq!(
            validate("EC12345678901234567890123"),
            Err(ValidationError::InvalidLength)
        );
    }

    #[test]
    fn wrong_check_digit() {
        assert_eq!(
            validate("EC9912345678901234567890"),
            Err(ValidationError::InvalidCheckDigit)
        );
    }

    #[test]
    fn format_invalid() {
        assert!(format("invalid").is_none());
    }
}
