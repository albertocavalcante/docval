# docval

`docval` is a Rust crate for document validation tailored for various countries.
The primary focus is to provide validation support for different types of documents, starting with Brazil's Tax IDs (`CPF` and `CNPJ`).

Additionally, the crate offers an optional integration with the popular `validator` crate for enhanced validation features.

> **WARNING:** This crate is still extensible and not stable. Expect breaking changes in future releases.

## Features

- **Brazilian Tax ID (CPF) Validator**: A dedicated validator for Brazilian CPF numbers.
- **Extensible**: Built with extensibility in mind, making it easy to add support for more document types and countries.
- **Optional Validator Integration**: Enable the `validator-integration` feature to integrate with the `validator` crate.

## Usage

Add `docval` to your `Cargo.toml`:

```toml
[dependencies]
docval = "0.1.0"
```

To enable the optional `validator` crate integration:

```toml
[dependencies]
docval = { version = "0.1.0", features = ["validator-integration"] }
```

### Example

Using the Brazilian CPF validator:

```rust
use docval::brazil::BrazilTaxIdValidator;

fn main() {
    let cpf = "123.456.789-09";
    match BrazilTaxIdValidator::validate(cpf) {
        Ok(_) => println!("Valid CPF!"),
        Err(err) => println!("Invalid CPF: {:?}", err),
    }
}
```

## Optional Features

### `validator-integration`

You can enable additional validation capabilities provided by the `validator` crate by enabling the `validator-interation` feature:

```toml
[dependencies]
docval = { version = "0.1.0", features = ["validator-integration"] }
```

Example with `validator` integration:

```rust
use validator::Validate;
use docval::brazil::BrazilTaxIdValidator;

#[derive(Debug, Validate)]
struct User {
    #[validate(custom(function="BrazilTaxIdValidator::validator"))]
    cpf: String,
}

fn main() {
    let user = User { cpf: "123.456.789-09".to_string() };
    match user.validate() {
        Ok(_) => println!("User is valid!"),
        Err(e) => println!("Validation errors: {:?}", e),
    }
}
```

## Contributing

Feel free to contribute by opening issues or submitting pull requests. Any additional document types or improvements are welcome.

## License

This project is licensed under the MIT License.
