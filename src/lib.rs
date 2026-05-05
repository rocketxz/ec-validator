//! Validation utilities for Ecuadorian government IDs and financial data.
//! 
//! This crate provides validation for:
//! - **Cédula de Identidad**: Ecuadorian national ID (10 digits)
//! - **RUC**: Registro Único de Contribuyentes (taxpayer ID)
//! - **IBAN**: International Bank Account Number for Ecuador
//! 
//! # Features
//! 
//! | Feature | Description | Default |
//! |---------|-------------|---------|
//! | `serde` | Enable serialization/deserialization derive macros | No |
//! | `wasm` | Enable WebAssembly bindings with wasm-bindgen | No |
//! 
//! # Validators
//! 
//! | Module | Document Type | Length | Algorithm |
//! |--------|-------------|--------|-----------|
//! | [`cedula`] | Cédula de Identidad | 10 digits | Mod-10 |
//! | [`ruc`] | RUC (Natural Person) | 13 digits | Mod-10 (first 10 as cédula) |
//! | [`ruc`] | RUC (Juridical Entity) | 13 digits | Mod-11 |
//! | [`ruc`] | RUC (Public Entity) | 13 digits | Mod-11 |
//! | [`iban`] | IBAN (Ecuador) | 24 chars | Mod-97 |
//! 
//! # Example
//! 
//! ```rust
//! use ec_validator::{cedula, ruc, iban};
//! 
//! // Validate an Ecuadorian Cédula
//! let cedula_result = cedula::validate("1713175071");
//! assert!(cedula_result.is_ok());
//! 
//! // Validate an Ecuadorian RUC
//! let ruc_result = ruc::validate("1713175071001");
//! assert!(ruc_result.is_ok());
//! 
//! // Validate an Ecuadorian IBAN
//! // Note: Use a properly formatted EC IBAN with valid check digits
//! let iban_result = iban::validate("EC8912345678901234567890");
//! // Result depends on check digit validity
//! ```
//! 
//! # Errors
//! 
//! All validators return a [`ValidationError`] enum indicating the specific failure reason:
//! - [`ValidationError::InvalidLength`] - Wrong number of digits/characters
//! - [`ValidationError::InvalidCheckDigit`] - Check digit validation failed
//! - [`ValidationError::InvalidProvinceCode`] - Invalid province code (not 01-24)
//! - [`ValidationError::InvalidRucType`] - Invalid RUC type indicator
//! - [`ValidationError::InvalidFormat`] - Invalid character format

pub mod cedula;
pub mod error;
pub mod iban;
pub mod ruc;

pub use error::ValidationError;
#[cfg(feature = "serde")]
pub use error::ValidationErrorWithContext;

#[cfg(feature = "wasm")]
pub mod wasm;