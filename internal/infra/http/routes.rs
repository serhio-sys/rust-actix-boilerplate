use std::sync::Arc;

use actix_web::{
    dev::{ ServiceFactory, ServiceRequest, ServiceResponse },
    middleware::from_fn,
    web::{ self, Data },
    HttpResponse,
    Responder,
    Scope,
};

use crate::container::container::Container;

use super::{
    controllers::{
        auth_controller::{ login, logout, register, AuthController },
        user_controller::{ get_user_by_id, get_users, UserController },
    },
    middlewares::auth_middleware::auth_middleware,
};

pub fn init_routes(cfg: &mut web::ServiceConfig, container: Arc<Container>) {
    let auth_controller_data = web::Data::new(container.controllers.user_controller.clone());
    let user_controller_data = web::Data::new(container.controllers.auth_controller.clone());
    init_user_routes(cfg, auth_controller_data, Arc::clone(&container));
    init_auth_routes(cfg, user_controller_data, Arc::clone(&container));
    cfg.default_service(web::get().to(not_found_handler));
}

async fn not_found_handler() -> impl Responder {
    return HttpResponse::NotFound().json("Not found 404");
}

fn init_auth_routes(
    cfg: &mut web::ServiceConfig,
    auth_controller: Data<AuthController>,
    container: Arc<Container>
) {
    cfg.app_data(auth_controller.clone()).service(
        web
            ::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .service(protected_route(container, "").route("/logout", web::post().to(logout)))
    );
}

fn init_user_routes(
    cfg: &mut web::ServiceConfig,
    us_controller: Data<UserController>,
    container: Arc<Container>
) {
    cfg.app_data(us_controller.clone()).service(
        protected_route(container, "/users")
            .route("{id}", web::get().to(get_user_by_id))
            .route("", web::get().to(get_users))
    );
}

fn protected_route(
    container: Arc<Container>,
    path: &str
) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = ()
    >
> {
    return web::scope(path).wrap(
        from_fn(move |req: ServiceRequest, next| {
            return auth_middleware(
                Arc::clone(&container.services.user_service),
                Arc::clone(&container.services.auth_service),
                req,
                next
            );
        })
    );
}
