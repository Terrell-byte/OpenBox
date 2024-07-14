use crate::Application::traits::T_UserRepository;
use crate::Domain::UserEntity::User;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;

pub struct UserRepository {
    db: Surreal<Client>,
}

impl UserRepository {
    pub async fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        self.db
            .insert("users", user)
            .await?;
        Ok(())
    }
}
