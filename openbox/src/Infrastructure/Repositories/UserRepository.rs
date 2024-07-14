use crate::Application::traits::T_UserRepository;
use crate::Domain::UserEntity::User;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use anyhow::Result;

pub struct UserRepository {
    db: Surreal<Client>,
}

impl UserRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl T_UserRepository for UserRepository {
    async fn create_user(&self, user: &User) -> Result<()> {
        self.db
            .create("users")
            .content(user)
            .await?;
        Ok(())
    }

    async fn get_user(&self, public_key: &str) -> Result<User> {
        let user: Option<User> = self.db
            .select(("users", public_key))
            .await?;
        user.ok_or_else(|| anyhow::anyhow!("User not found"))
    }
}