/// Brazil Tax ID Validator
///
/// This module provides functionality to validate Brazilian Tax Identification
/// Numbers (CPF and CNPJ). CPF (Cadastro de Pessoas Físicas) is used for individuals,
/// and CNPJ (Cadastro Nacional da Pessoa Jurídica) is used for companies.
///
/// The `BrazilTaxIdValidator` struct contains methods to sanitize input, validate the length,
/// check for equal digits, validate check digits, and ultimately determine if a given
/// CPF or CNPJ is valid.
///
/// # Usage
///
/// ```
/// use docval::brazil::BrazilTaxIdValidator;
///
/// let cpf = "123.456.789-09";
/// let cnpj = "12.345.678/0001-95";
///
/// assert!(BrazilTaxIdValidator::is_valid(cpf).is_ok());
/// assert!(BrazilTaxIdValidator::is_valid(cnpj).is_ok());
/// ```
///
/// # Methods
///
/// - `is_valid(value: &str) -> Result<(), &'static str>`:
///   Main entry point to validate a given CPF or CNPJ. Removes non-digit characters,
///   checks for length consistency, and validates check digits.
///
/// - `sanitize_input(value: &str) -> String`:
///   Removes non-digit characters from the input.
///
/// - `validate(value: &str, length: usize, weights: &[u32]) -> Result<(), &'static str>`:
///   Checks the validity of the CPF or CNPJ based on length and check digits.
///
/// - `has_all_equal_digits(value: &str) -> bool`:
///   Checks if all characters in the input are the same.
///
/// - `is_valid_check_digits(value: &str, length: usize, weights: &[u32]) -> bool`:
///   Validates the check digits for the given CPF or CNPJ.
///
/// - `calculate_check_digit(value: &str, weights: &[u32]) -> u32`:
///   Calculates a single check digit for CPF or CNPJ.
///
/// These methods ensure that the input CPF or CNPJ is rigorously validated according
/// to Brazilian standards.
#[cfg(feature = "validator-integration")]
use validator::ValidationError;
pub struct BrazilTaxIdValidator;

const CPF_STANDARD_LENGTH: usize = 11;
const CNPJ_STANDARD_LENGTH: usize = 14;
const CPF_MULTIPLIER_WEIGHTS: &[u32] = &[11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
const CNPJ_MULTIPLIER_WEIGHTS: &[u32] = &[6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
const VALIDATION_MODULUS: u32 = 11;

impl BrazilTaxIdValidator {
    /// Validates if the given CPF or CNPJ is correct according to the Bra zilian standards.
    /// The input can be a plain or formatted string (with dots, slashes, or hyphens).
    pub fn is_valid(value: &str) -> Result<(), &'static str> {
        let sanitized_value = Self::sanitize_input(value);
        if sanitized_value.is_empty() {
            return Err("Invalid input");
        }
        match sanitized_value.len() {
            CPF_STANDARD_LENGTH => Self::validate(&sanitized_value, CPF_STANDARD_LENGTH, CPF_MULTIPLIER_WEIGHTS),
            CNPJ_STANDARD_LENGTH => Self::validate(&sanitized_value, CNPJ_STANDARD_LENGTH, CNPJ_MULTIPLIER_WEIGHTS),
            _ => Err("Invalid length"),
        }
    }

    #[cfg(feature = "validator-integration")]
    /// Custom function to validate CPF and CNPJ using the TaxIdValidator.
    pub fn validator(value: &str) -> Result<(), ValidationError> {
        match Self::is_valid(value) {
            Ok(()) => Ok(()),
            Err(err_msg) => Err(ValidationError::new(err_msg)),
        }
    }

    /// Removes non-digit characters from the input.
    fn sanitize_input(value: &str) -> String {
        value.chars().filter(char::is_ascii_digit).collect()
    }

    /// Checks if the CPF or CNPJ is valid based on length and check digits.
    fn validate(value: &str, length: usize, weights: &[u32]) -> Result<(), &'static str> {
        if Self::has_all_equal_digits(value) {
            return Err("All digits are equal");
        }
        if Self::is_valid_check_digits(value, length, weights) {
            Ok(())
        } else {
            Err("Invalid checksum")
        }
    }

    /// Checks if all characters in the input are the same.
    fn has_all_equal_digits(value: &str) -> bool {
        let mut chars = value.chars();
        match chars.next() {
            Some(first_char) => chars.all(|c| c == first_char),
            None => false,
        }
    }

    /// Validates the check digits for CPF or CNPJ.
    fn is_valid_check_digits(value: &str, length: usize, weights: &[u32]) -> bool {
        let check_digits = &value[length - 2..];
        let calculated_first_digit = Self::calculate_check_digit(&value[..length - 2], &weights[1..]);
        let calculated_second_digit = Self::calculate_check_digit(&value[..length - 1], weights);
        let calculated_check_digits = format!("{}{}", calculated_first_digit, calculated_second_digit);
        check_digits == calculated_check_digits
    }

    fn calculate_check_digit(value: &str, weights: &[u32]) -> u32 {
        assert_eq!(value.chars().count(), weights.len());

        let sum: u32 = value
            .chars()
            .zip(weights.iter())
            .map(|(c, &w)|
                c.to_digit(10).expect("Invalid digit in input") * w
            )
            .sum();

        let remainder = sum % VALIDATION_MODULUS;
        if remainder < 2 {
            0
        } else {
            VALIDATION_MODULUS - remainder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_all_equal_digits() {
        assert!(BrazilTaxIdValidator::has_all_equal_digits("11111111111"));
    }
}
