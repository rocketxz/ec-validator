use crate::cedula;
use crate::ValidationError;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RucType {
    NaturalPerson,
    JuridicalEntity,
    PublicEntity,
}

/// Determines the RUC type from the 3rd digit.
///
/// # Arguments
///
/// * `input` - A 13-digit RUC string.
///
/// # Returns
///
/// Returns `Some` with the [`RucType`] if valid, `None` otherwise.
///
/// # Examples
///
/// ```
/// use ec_validator::ruc::{ruc_type, RucType};
///
/// assert_eq!(ruc_type("1713175071001"), Some(RucType::NaturalPerson));
/// assert_eq!(ruc_type("1790085783001"), Some(RucType::JuridicalEntity));
/// ```
pub fn ruc_type(input: &str) -> Option<RucType> {
    let input = input.trim();
    if input.len() != 13 || !input.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    match input.chars().nth(2).unwrap().to_digit(10).unwrap() {
        0..=5 => Some(RucType::NaturalPerson),
        6 => Some(RucType::PublicEntity),
        9 => Some(RucType::JuridicalEntity),
        _ => None,
    }
}

/// Validates an Ecuadorian RUC (Registro Único de Contribuyentes).
///
/// # Arguments
///
/// * `input` - A string containing the RUC number (13 digits).
///
/// # Errors
///
/// Returns [`ValidationError`] on validation failure:
/// - [`ValidationError::InvalidLength`] - Not exactly 13 digits
/// - [`ValidationError::InvalidFormat`] - Non-numeric or invalid RUC type digit
/// - [`ValidationError::InvalidProvinceCode`] - Province code not in 01-24
/// - [`ValidationError::InvalidCheckDigit`] - Check digit invalid
///
/// # Examples
///
/// ```
/// use ec_validator::ruc;
///
/// // Natural person RUC (3rd digit 0-5)
/// let result = ruc::validate("1713175071001");
/// assert!(result.is_ok());
///
/// // Juridical entity RUC (3rd digit 9)
/// let result = ruc::validate("1790085783001");
/// assert!(result.is_ok());
///
/// // Public entity RUC (3rd digit 6)
/// let result = ruc::validate("1760001550001");
/// assert!(result.is_ok());
/// ```
pub fn validate(input: &str) -> Result<(), ValidationError> {
    let input = input.trim();

    if input.len() != 13 {
        return Err(ValidationError::InvalidLength);
    }

    if !input.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidFormat);
    }

    let third_digit = input.chars().nth(2).unwrap().to_digit(10).unwrap();

    match third_digit {
        0..=5 => validate_natural(input),
        6 => validate_public(input),
        7..=8 => Err(ValidationError::InvalidFormat),
        9 => validate_juridical(input),
        _ => Err(ValidationError::InvalidFormat),
    }
}

fn validate_natural(input: &str) -> Result<(), ValidationError> {
    let cedula = &input[..10];
    cedula::validate(cedula)?;

    let establishment: u32 = input[10..13].parse().unwrap();
    if establishment == 0 || establishment > 999 {
        return Err(ValidationError::InvalidFormat);
    }

    Ok(())
}

fn validate_juridical(input: &str) -> Result<(), ValidationError> {
    let province: u32 = input[..2].parse().unwrap();
    if province == 0 || province > 24 {
        return Err(ValidationError::InvalidProvinceCode);
    }

    let check_digit_pos9 = input.chars().nth(9).unwrap().to_digit(10).unwrap();

    let weights = [4, 3, 2, 7, 6, 5, 4, 3, 2];
    let mut sum = 0u32;

    for i in 0..9 {
        let digit = input.chars().nth(i).unwrap().to_digit(10).unwrap();
        sum += digit * weights[i];
    }

    let computed_check = if sum % 11 == 0 { 0 } else { 11 - (sum % 11) };

    if computed_check == 10 {
        return Err(ValidationError::InvalidCheckDigit);
    }

    if computed_check != check_digit_pos9 {
        return Err(ValidationError::InvalidCheckDigit);
    }

    let establishment: u32 = input[9..13].parse().unwrap();
    if establishment == 0 || establishment > 9999 {
        return Err(ValidationError::InvalidFormat);
    }

    Ok(())
}

fn validate_public(input: &str) -> Result<(), ValidationError> {
    let province: u32 = input[..2].parse().unwrap();
    if province == 0 || province > 24 {
        return Err(ValidationError::InvalidProvinceCode);
    }

    let check_digit = input.chars().nth(8).unwrap().to_digit(10).unwrap();

    let weights = [3, 2, 7, 6, 5, 4, 3, 2];
    let mut sum = 0u32;

    for i in 0..8 {
        let digit = input.chars().nth(i).unwrap().to_digit(10).unwrap();
        sum += digit * weights[i];
    }

    let remainder = if sum % 11 == 0 { 0 } else { 11 - (sum % 11) };

    if remainder != check_digit {
        return Err(ValidationError::InvalidCheckDigit);
    }

    let establishment: u32 = input[9..13].parse().unwrap();
    if establishment == 0 || establishment > 9999 {
        return Err(ValidationError::InvalidFormat);
    }

    Ok(())
}

/// Convenience function that returns `true` if the RUC is valid, `false` otherwise.
///
/// # Arguments
///
/// * `input` - A string containing the RUC number.
///
/// # Examples
///
/// ```
/// use ec_validator::ruc;
///
/// assert!(ruc::is_valid("1713175071001"));
/// assert!(!ruc::is_valid("0000000000000"));
/// ```
pub fn is_valid(input: &str) -> bool {
    validate(input).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_ruc() {
        assert!(validate("1713175071001").is_ok());
    }

    #[test]
    fn juridical_ruc() {
        assert!(validate("1790085783001").is_ok());
    }

    #[test]
    fn public_ruc() {
        assert!(validate("1760001550001").is_ok());
    }

    #[test]
    fn invalid_establishment() {
        assert_eq!(
            validate("1713175071000"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn bad_check_digit() {
        assert_eq!(
            validate("1790085782001"),
            Err(ValidationError::InvalidCheckDigit)
        );
    }
}

