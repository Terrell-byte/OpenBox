use anyhow::Result;
use Crate::Application::traits::T_Protocol;

pub struct Protocol {
}
#[async_trait::async_trait]
impl T_Protocol for Protocol {
    fn generate_payload(&self) -> Result<String> {
        Ok("generated_payload".to_string())
    }

    fn verify_signature(&self, public_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool> {
        Ok(true)
    }
}