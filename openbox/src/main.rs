use crate::infrastructure::database::init_db;
use crate::config::Config;

fn main() {
    let config = Config::load().unwrap().expect("Failed to load configuration");
    let db = init_db(&config).unwrap().expect("Failed to initialize database connection");  
}
