use std::sync::{ Arc, Mutex };
use config::CONFIGURATION;
use diesel::{ r2d2::{ ConnectionManager, Pool }, PgConnection };

use crate::{
    infra::{
        database::{ migration::migrate, user_repository::UserRepository },
        http::controllers::user_controller::UserController,
    },
    service::user_service::UserService,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Container {
    services: Arc<Services>,
    pub controllers: Controllers,
}

#[derive(Clone)]
struct Services {
    us_service: Arc<Mutex<UserService>>,
}
#[derive(Clone)]
pub struct Controllers {
    pub us_controller: UserController,
}

pub fn new() -> Container {
    let manager = get_database_connection();
    let pool = Pool::builder().build(manager).expect("Error in creating pool");

    migrate(&mut pool.get().unwrap(), &CONFIGURATION.migration_location);

    let user_repo = UserRepository { pool: pool };
    let services: Arc<Services> = Arc::new(Services {
        us_service: UserService::new(user_repo),
    });
    let controllers: Controllers = Controllers {
        us_controller: UserController::new(services.us_service.clone()),
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
