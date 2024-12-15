use std::sync::Arc;

use rust_commons::config::database_config::DatabaseConfig;
use rust_commons::config::{ get_var, get_var_or_default };
use rust_commons::dotenvy::dotenv;
use rust_commons::lazy_static::lazy_static;

pub use rust_commons::log;
pub use rust_commons::logger::init_logger;

lazy_static! {
    pub static ref CONFIGURATION: Configuration = {
        return get_configuration();
    };
}

pub struct Configuration {
    pub database_name: String,
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub migration_location: String,
    pub migration_version: String,
    pub file_storage_location: String,
    pub jwt_ttl: u64,
    pub jwt_secret: String,
}

impl DatabaseConfig for Configuration {
    fn get_db_host(&self) -> std::sync::Arc<str> {
        return Arc::from(self.database_host.clone());
    }

    fn get_db_name(&self) -> std::sync::Arc<str> {
        return Arc::from(self.database_name.clone());
    }

    fn get_db_password(&self) -> std::sync::Arc<str> {
        return Arc::from(self.database_password.clone());
    }

    fn get_db_user(&self) -> std::sync::Arc<str> {
        return Arc::from(self.database_user.clone());
    }

    fn get_migrations_location(&self) -> std::sync::Arc<str> {
        return Arc::from(self.migration_location.clone());
    }

    fn get_migrations_version(&self) -> std::sync::Arc<str> {
        return Arc::from(self.migration_version.clone());
    }
}

fn get_configuration() -> Configuration {
    if let Err(exc) = dotenv() {
        log::error!("Error in loading .env file - [{}]", exc.to_string());
    }
    return Configuration {
        database_name: get_var("DATABASE_NAME"),
        database_host: get_var("DATABASE_HOST"),
        database_user: get_var("DATABASE_USER"),
        database_password: get_var("DATABASE_PASSWORD"),
        migration_location: get_var("MIGRATION_LOCATION"),
        // 2024-09-21-122416 - example of migration verison.
        // latest - for running migration to last one in migrations folder.
        migration_version: get_var_or_default("MIGRATE_TO", "latest"),
        file_storage_location: get_var_or_default("FILE_STORAGE_LOCATION", "file_storage"),
        jwt_ttl: 72 * 3600,
        jwt_secret: get_var_or_default("JWT_SECRET", "1234567890"),
    };
}
