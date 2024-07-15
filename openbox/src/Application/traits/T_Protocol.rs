use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait T_Protocol {
    async fn generate_payload(&self) -> Result<Vec<u8>>;
    async fn verify_signature(&self, public_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool>;
    async fn salt_data(&self, payload: &[u8], data: &[u8]) -> Result<Data>;
}