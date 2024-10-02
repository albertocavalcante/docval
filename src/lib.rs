pub mod brazil {
    pub mod tax_id; // Declare the tax_id module within brazil
    pub use tax_id::BrazilTaxIdValidator; // Re-export if needed
}
