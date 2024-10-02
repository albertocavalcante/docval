use docval::brazil::BrazilTaxIdValidator;

fn main() {
    let cpf = "123.456.789-09";
    match BrazilTaxIdValidator::is_valid(cpf) {
        Ok(_) => println!("CPF is valid!"),
        Err(err) => println!("CPF is invalid: {}", err),
    }

    let cnpj = "12.345.678/0001-95";
    match BrazilTaxIdValidator::is_valid(cnpj) {
        Ok(_) => println!("CNPJ is valid!"),
        Err(err) => println!("CNPJ is invalid: {}", err),
    }
}
