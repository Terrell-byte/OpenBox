use crate::config::Config;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub async fn init_db(config: &Config) -> Result<Surreal<Client>, Box<dyn std::error::Error>> {
    let connection_url = format!("{}:{}", config.database.host, config.database.port);

    let db = Surreal::new::<Ws>(connection_url).await?;

    db.signin(Root {
        username: &config.database.user,
        password: &config.database.pass,
    })
    .await?;

    db.use_ns(&config.database.namespace).use_db(&config.database.db_name).await?;

    Ok(db)
}