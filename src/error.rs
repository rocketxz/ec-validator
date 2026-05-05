use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ValidationError {
    InvalidLength,
    InvalidCheckDigit,
    InvalidProvinceCode,
    InvalidRucType,
    InvalidFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidationErrorWithContext {
    pub error: ValidationError,
    pub context: Option<&'static str>,
}

impl ValidationError {
    pub fn with_context(self, context: &'static str) -> ValidationErrorWithContext {
        ValidationErrorWithContext {
            error: self,
            context: Some(context),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidLength => write!(f, "invalid length"),
            ValidationError::InvalidCheckDigit => write!(f, "invalid check digit"),
            ValidationError::InvalidProvinceCode => write!(f, "invalid province code"),
            ValidationError::InvalidRucType => write!(f, "invalid RUC type"),
            ValidationError::InvalidFormat => write!(f, "invalid format"),
        }
    }
}

impl fmt::Display for ValidationErrorWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = self.context {
            write!(f, "{}: {}", ctx, self.error)
        } else {
            write!(f, "{}", self.error)
        }
    }
}

impl std::error::Error for ValidationError {}
impl std::error::Error for ValidationErrorWithContext {}
