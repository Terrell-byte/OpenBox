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
    pub async fn get_user(&self, public_key: &str) -> Result<User, Box<dyn std::error::Error>> {
        let user: Option<User> = self.db
            .select(("users", public_key))
            .await?;
        user.ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "!USER NOT FOUND!")))     
    }
}
