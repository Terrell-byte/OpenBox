use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait T_Protocol {
    async fn generate_payload(&self) -> Result<String>;
    async fn verify_signature(&self, payload: &str, signature: &str) -> Result<bool>;
}