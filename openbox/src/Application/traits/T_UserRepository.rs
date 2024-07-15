use crate::Domain::UserEntity::User;
use async_trait::async_trait;

#[async_trait]
pub trait T_UserRepository {
    pub async fn create_user(&self, user: &User) -> Result<()>;
    pub async fn get_user(&self, public_key: &str) -> Result<User>;
}