use docval::brazil::BrazilTaxIdValidator;

#[test]
fn test_valid_cpf() {
    let cpf = "12345678909";
    assert!(BrazilTaxIdValidator::is_valid(cpf).is_ok());
}

#[test]
fn test_valid_cpf_with_formatting() {
    let cpf = "123.456.789-09";
    assert!(BrazilTaxIdValidator::is_valid(cpf).is_ok());
}

#[test]
fn test_invalid_cpf() {
    let cpf = "000.000.000-00";
    assert!(BrazilTaxIdValidator::is_valid(cpf).is_err());
}

#[test]
fn test_invalid_cpf_length() {
    let cpf = "123.456.789";
    assert!(BrazilTaxIdValidator::is_valid(cpf).is_err());
}

#[test]
fn test_valid_cnpj() {
    let cnpj = "12.345.678/0001-95";
    assert!(BrazilTaxIdValidator::is_valid(cnpj).is_ok());
}

#[test]
fn test_invalid_cnpj() {
    let cnpj = "00.000.000/0000-00";
    assert!(BrazilTaxIdValidator::is_valid(cnpj).is_err());
}

#[test]
fn test_invalid_cnpj_length() {
    let cnpj = "12.345.678/0001";
    assert!(BrazilTaxIdValidator::is_valid(cnpj).is_err());
}


#[test]
fn test_invalid_length() {
    assert!(BrazilTaxIdValidator::is_valid("123").is_err());
}


#[test]
fn test_valid_cnpj_with_formatting() {
    assert!(BrazilTaxIdValidator::is_valid("12.345.678/0001-95").is_ok());
}

#[test]
fn test_invalid_characters() {
    assert!(BrazilTaxIdValidator::is_valid("123.abc.789-0x").is_err());
}

#[cfg(feature = "validator-integration")]
#[test]
fn test_validate_cpf_cnpj() {
    let valid_cpf = "123.456.789-09";
    let invalid_cnpj = "00.000.000/0000-00";
    assert!(BrazilTaxIdValidator::validator(valid_cpf).is_ok());
    assert!(BrazilTaxIdValidator::validator(invalid_cnpj).is_err());
}
