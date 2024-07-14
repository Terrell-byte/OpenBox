use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait T_Protocol {
    async fn generate_payload(&self) -> Result<String>;
    async fn verify_signature(&self, public_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool>;
}