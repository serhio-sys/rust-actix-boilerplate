use std::sync::Arc;

use actix_web::{ web, Responder };

use crate::container::container::Container;

use super::controllers::user_controller::UserController;

pub fn init_routes(cfg: &mut web::ServiceConfig, container: Arc<Container>) {
    init_user_routes(cfg, container.controllers.us_controller.clone());
}

fn init_user_routes(cfg: &mut web::ServiceConfig, us_controller: UserController) {
    let us_controller = web::Data::new(us_controller);
    cfg.app_data(us_controller.clone()).service(
        web::scope("/users").route("", web::get().to(get_users))
    );
}

async fn get_users(us_controller: web::Data<UserController>) -> impl Responder {
    return us_controller.get_users().await;
}
