use anyhow::Result;
use ed25519::signature::{Signer, Verifier};
use rand::RngCore;
use chrono::Utc;
use Crate::Application::traits::T_Protocol;


pub struct Protocol {
}
#[async_trait::async_trait]
impl T_Protocol for Protocol {
    fn generate_payload(&self) -> Result<Vec<u8>> {
        let mut payload = Vec::with_capacity(40);
        
        let mut nonce = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut nonce);
        payload.extend_from_slice(&nonce);

        let timestamp = Utc::now().timestamp() as u64;
        payload.extend_from_slice(&timestamp.to_be_bytes());

        Ok(payload)
    }

    fn verify_signature(&self, public_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool> {
        let public_key = ed25519::PublicKey::from_bytes(public_key)?;
        let signature = ed25519::Signature::from_bytes(signature)?;
        public_key.verify(payload, &signature).is_ok()
    }

    // TODO: Refactor this
    fn salt_data(&self, payload: &[u8], data: &[u8]) -> Result<Data> {
        let mut data = data.to_vec();
        data.extend_from_slice(payload);
        Ok(data)
    }
}