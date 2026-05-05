use crate::ValidationError;

/// Validates an Ecuadorian Cédula de Identidad (national ID).
///
/// # Arguments
/// 
/// * `input` - A string containing the Cédula number (10 digits).
///
/// # Errors
/// 
/// Returns [`ValidationError`] on validation failure:
/// - [`ValidationError::InvalidLength`] - Not exactly 10 digits
/// - [`ValidationError::InvalidFormat`] - Contains non-numeric characters
/// - [`ValidationError::InvalidProvinceCode`] - Province code not in 01-24
/// - [`ValidationError::InvalidCheckDigit`] - Mod-10 check digit invalid
///
/// # Examples
///
/// ```
/// use ec_validator::cedula;
///
/// // Valid Ecuadorian Cédula
/// let result = cedula::validate("1713175071");
/// assert!(result.is_ok());
///
/// // Invalid Cédula (wrong check digit)
/// let result = cedula::validate("1713175072");
/// assert!(result.is_err());
/// ```
pub fn validate(input: &str) -> Result<(), ValidationError> {
    let input = input.trim();
    
    if input.len() != 10 {
        return Err(ValidationError::InvalidLength);
    }
    
    if !input.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidFormat);
    }
    
    let province: u32 = input[..2].parse().unwrap();
    if province == 0 || province > 24 {
        return Err(ValidationError::InvalidProvinceCode);
    }
    
    let third_digit = input.chars().nth(2).unwrap().to_digit(10).unwrap();
    if third_digit > 6 {
        return Err(ValidationError::InvalidFormat);
    }
    
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    
    let mut sum = 0u32;
    for (i, &digit) in digits.iter().enumerate().take(9) {
        let product = if i % 2 == 0 {
            digit * 2
        } else {
            digit
        };
        if product >= 10 {
            sum += product - 9;
        } else {
            sum += product;
        }
    }
    
    let remainder = if sum.is_multiple_of(10) {
        0
    } else {
        10 - (sum % 10)
    };
    
    if remainder != digits[9] {
        return Err(ValidationError::InvalidCheckDigit);
    }
    
    Ok(())
}

/// Convenience function that returns `true` if the Cédula is valid, `false` otherwise.
///
/// # Arguments
///
/// * `input` - A string containing the Cédula number.
///
/// # Examples
///
/// ```
/// use ec_validator::cedula;
///
/// assert!(cedula::is_valid("1713175071"));
/// assert!(!cedula::is_valid("0000000000"));
/// ```
pub fn is_valid(input: &str) -> bool {
    validate(input).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn valid_cedulas() {
        assert!(validate("1713175071").is_ok());
        assert!(validate("0910009000").is_ok());
        assert!(validate("1710034065").is_ok());
    }

    #[test]
    fn invalid_length() {
        assert_eq!(validate("171317507"), Err(ValidationError::InvalidLength));
    }

    #[test]
    fn invalid_province() {
        assert_eq!(validate("9913175071"), Err(ValidationError::InvalidProvinceCode));
    }

    #[test]
    fn invalid_check_digit() {
        assert_eq!(validate("1713175072"), Err(ValidationError::InvalidCheckDigit));
    }

    #[test]
    fn non_numeric() {
        assert!(validate("171317507a").is_err());
    }

    proptest! {
        #[test]
        fn no_panic(s in "\\d{10}") {
            let _ = validate(&s);
        }
    }
}