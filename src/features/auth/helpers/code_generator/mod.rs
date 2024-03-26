use rand::Rng;

use crate::BaseError;

#[derive(Debug)]
pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_random_code(length: u8) -> Result<String, BaseError> {
        tracing::info!("Generating code");
        let mut range = rand::thread_rng();

        const CHARSET: &[u8] = b"0123456789";
        let code: String = (0..length)
            .map(|_| {
                let idx = range.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        Ok(code)
    }
}
