# ec-validator

[![crates.io](https://img.shields.io/crates/v/ec-validator.svg)](https://crates.io/crates/ec-validator)
[![docs.rs](https://docs.rs/ec-validator/badge.svg)](https://docs.rs/ec-validator)
[![CI](https://github.com/rocketxz/ec-validator/workflows/CI/badge.svg)](https://github.com/rocketxz/ec-validator/actions)
[![Made in Ecuador](https://img.shields.io/badge/Made_in-Ecuador-FF6F00?styleflat&logo=flag)](https://en.wikipedia.org/wiki/Ecuador)
> [!NOTE]
> Ecuador does not currently use IBAN in practice.
> Banks use CCI instead. IBAN support is included for ISO compliance.

High-performance validation for Ecuadorian government IDs (cédula, RUC) and financial data (IBAN).

## Quick install

```bash
cargo add ec-validator
```

## Example

```rust
use ec_validator::{cedula, ruc, iban};

let cedula = cedula::validate("1713175071").is_ok();           // true
let ruc    = ruc::validate("1790085783001").is_ok();          // true (juridical)
let rtype  = format!("{:?}", ruc::ruc_type("1790085783001")); // JuridicalEntity
let iban   = iban::validate("EC8912345678901234567890").is_ok(); // depends on check digits
```

## Features

| Feature | Description |
|--------|-------------|
| Cédula | 10-digit national ID (Mod-10) |
| RUC | 13-digit taxpayer ID: Natural Person, Juridical Entity, Public Entity |
| IBAN | ISO 13616 (Mod-97) |

## Error types

| Variant | Description |
|---------|-------------|
| InvalidLength | Wrong number of digits/characters |
| InvalidCheckDigit | Check digit validation failed |
| InvalidProvinceCode | Invalid province code (not 01-24) |
| InvalidRucType | Invalid RUC type indicator |
| InvalidFormat | Invalid character format |

## Feature flags

| Flag | Description | Default |
|------|-------------|---------|
| `serde` | Enable serialization/deserialization | No |
| `wasm` | Enable WebAssembly bindings | No |

## Coming soon

- **CCI** (Código de Cuenta Interbancaria) validation

## Contributing

Contributions welcome! Specifically seeking help implementing bank account algorithms (local and interbank). Contact the maintainers for details.

## License

MIT
