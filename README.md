# ec-validator

[![crates.io](https://img.shields.io/crates/v/ec-validator)](https://crates.io/crates/ec-validator)
[![docs.rs](https://img.shields.io/docsrs/ec-validator/latest)](https://docs.rs/ec-validator/latest/ec_validator/)
[![CI](https://github.com/rocketxz/ec-validator/workflows/CI/badge.svg)](https://github.com/rocketxz/ec-validator/actions)

High-performance validation library for Ecuadorian government IDs and financial data.

## Quick Start

```rust
use ec_validator::{cedula, ruc, iban};

// Validate a Cédula (national ID)
assert!(cedula::validate("1713175071").is_ok());

// Validate a RUC (taxpayer ID)
assert!(ruc::validate("1713175071001").is_ok());

// Validate an Ecuadorian IBAN
assert!(iban::validate("EC8912345678901234567890").is_ok());
```

## Features

| Feature | Description | Default |
|--------|-------------|---------|
| `serde` | Enable Serialize/Deserialize derives | No |
| `wasm` | Enable WebAssembly bindings | No |

## Validation Rules

### Cédula de Identidad (10 digits)
The Ecuadorian national ID uses a Mod-10 algorithm. First two digits must be a valid province code (01–24). Third digit must be 0–6 for natural persons. The tenth digit is a check digit calculated using alternating weights [2,1,2,1,2,1,2,1,2].

### RUC (13 digits)
The Registro Único de Contribuyentes has three variants:
- **Natural Person** (3rd digit 0–5): First 10 digits must pass Cédula validation; establishment code (digits 11–13) must be 001–999.
- **Juridical Entity** (3rd digit 9): Province code 01–24; check digit at position 10 using Mod-11 weights [4,3,2,7,6,5,4,3,2]; establishment 0001–9999.
- **Public Entity** (3rd digit 6): Province code 01–24; check digit at position 9 using Mod-11 weights [3,2,7,6,5,4,3,2]; establishment 0001–9999.

### IBAN (24 characters)
Ecuadorian IBANs follow ISO 13616. Must start with "EC", be exactly 24 characters, and pass Mod-97 validation after rearranging (first 4 characters moved to end).

## WebAssembly

```bash
wasm-pack build --target web --features wasm
```

## Contributing

Test data is sourced from publicly available Ecuadorian government ID validation references including SRI specifications and international IBAN standards. When contributing new validation rules or test data, please cite relevant official sources.

## License

MIT
