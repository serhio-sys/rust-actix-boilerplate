use std::sync::{ Arc, RwLock };
use config::CONFIGURATION;
use diesel::{ r2d2::{ ConnectionManager, Pool }, PgConnection };

use crate::{
    filesystem::image_storage_service::ImageStorageService,
    infra::{
        database::{ session_repository::SessionRepository, user_repository::UserRepository },
        http::controllers::{ auth_controller::AuthController, user_controller::UserController },
    },
    services::{ auth_service::AuthService, user_service::UserService },
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Container {
    pub services: Arc<Services>,
    pub controllers: Controllers,
}

#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}
#[derive(Clone)]
pub struct Controllers {
    pub user_controller: UserController,
    pub auth_controller: AuthController,
}

pub fn new() -> Result<Container, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let manager = get_database_connection();
    let pool = Pool::builder()
        .max_size(5)
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)?;

    let pool = Arc::new(RwLock::new(pool));

    let user_repository = UserRepository::new(Arc::clone(&pool));
    let session_repository = SessionRepository::new(Arc::clone(&pool));
    let file_service = Arc::new(ImageStorageService::new(&CONFIGURATION.file_storage_location));
    let services: Arc<Services> = Arc::new(Services {
        user_service: UserService::new(
            Arc::clone(&user_repository),
            Arc::clone(&session_repository),
            Arc::clone(&file_service)
        ),
        auth_service: AuthService::new(
            Arc::clone(&user_repository),
            Arc::clone(&session_repository),
            Arc::clone(&file_service)
        ),
    });
    let controllers: Controllers = Controllers {
        user_controller: UserController::new(Arc::clone(&services.user_service)),
        auth_controller: AuthController::new(Arc::clone(&services.auth_service)),
    };
    let container = Container { services: services, controllers: controllers };
    return Ok(container);
}

fn get_database_connection() -> ConnectionManager<PgConnection> {
    let connection = ConnectionManager::<PgConnection>::new(
        &format!(
            "postgres://{}:{}@{}/{}?sslmode=disable",
            CONFIGURATION.database_user,
            CONFIGURATION.database_password,
            CONFIGURATION.database_host,
            CONFIGURATION.database_name
        )
    );
    return connection;
}
