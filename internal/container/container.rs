use std::sync::{ Arc, Mutex };
use config::CONFIGURATION;
use diesel::{ r2d2::{ ConnectionManager, Pool }, PgConnection };

use crate::{
    infra::{
        database::{
            migration::migrate,
            session_repository::SessionRepository,
            user_repository::UserRepository,
        },
        http::controllers::{ auth_controller::AuthController, user_controller::UserController },
    },
    services::{ auth_service::AuthService, user_service::UserService },
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Container {
    services: Arc<Services>,
    pub controllers: Controllers,
}

#[derive(Clone)]
struct Services {
    user_service: Arc<Mutex<UserService>>,
    auth_service: Arc<Mutex<AuthService>>,
}
#[derive(Clone)]
pub struct Controllers {
    pub user_controller: UserController,
    pub auth_controller: AuthController,
}

pub fn new() -> Container {
    let manager = get_database_connection();
    let pool = Pool::builder()
        .max_size(5)
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)
        .unwrap_or_else(|_| panic!("Error: Unable to establish database connection."));
    migrate(&mut pool.get().unwrap(), &CONFIGURATION.migration_location);

    let pool = Arc::new(pool);

    let user_repository = UserRepository::new(Arc::clone(&pool));
    let session_repository = SessionRepository::new(Arc::clone(&pool));
    let services: Arc<Services> = Arc::new(Services {
        user_service: UserService::new(Arc::clone(&user_repository)),
        auth_service: AuthService::new(
            Arc::clone(&user_repository),
            Arc::clone(&session_repository)
        ),
    });
    let controllers: Controllers = Controllers {
        user_controller: UserController::new(services.user_service.clone()),
        auth_controller: AuthController::new(services.auth_service.clone()),
    };
    let container = Container { services: services, controllers: controllers };
    return container;
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
