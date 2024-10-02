use validator::Validate;
use docval::brazil::BrazilTaxIdValidator;

#[derive(Debug, Validate)]
struct Citizen {
    #[validate(custom(function="BrazilTaxIdValidator::validator"))]
    cpf: String
}

#[derive(Debug, Validate)]
struct Company {
    #[validate(custom(function="BrazilTaxIdValidator::validator"))]
    cnpj: String,
}

fn main() {
    let valid_citizen = Citizen { cpf: "123.456.789-09".to_string() };
    match valid_citizen.validate() {
        Ok(_) => println!("User is valid!"),
        Err(e) => println!("Validation errors: {:?}", e),
    }

    let valid_company = Company { cnpj: "12.345.678/0001-95".to_string() };
    match valid_company.validate() {
        Ok(_) => println!("Company is valid!"),
        Err(e) => println!("Validation errors: {:?}", e)
    }

    let invalid_citizen = Citizen { cpf: "000.000.000-00".to_string() };
    match invalid_citizen.validate() {
        Ok(_) => println!("User is valid!"),
        Err(e) => println!("Validation errors: {:?}", e),
    }

    let invalid_company = Company { cnpj: "12.345.678/0001-99".to_string() };
    match invalid_company.validate() {
        Ok(_) => println!("Company is valid!"),
        Err(e) => println!("Validation errors: {:?}", e)
    }
}
