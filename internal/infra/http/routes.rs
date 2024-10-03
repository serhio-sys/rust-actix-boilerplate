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

const BASIC_PATH: &str = "/api/v1";

use super::{
    controllers::{
        auth_controller::{ login, logout, register, AuthController },
        user_controller::{ delete, find_all, find_me, UserController },
    },
    middlewares::auth_middleware::auth_middleware,
};

pub fn init_routes(cfg: &mut web::ServiceConfig, container: Arc<Container>) {
    let auth_controller_data = web::Data::new(container.controllers.user_controller.clone());
    let user_controller_data = web::Data::new(container.controllers.auth_controller.clone());
    cfg.service(
        web
            ::scope(BASIC_PATH)
            .service(init_auth_routes(user_controller_data, Arc::clone(&container)))
            .service(init_user_routes(auth_controller_data, Arc::clone(&container)))
    );
    cfg.service(
        web::scope("/api").route(
            "",
            web::head().to(move || async move {
                return HttpResponse::Ok().finish().map_into_boxed_body();
            })
        )
    );
    cfg.default_service(web::get().to(not_found_handler));
}

async fn not_found_handler() -> impl Responder {
    return HttpResponse::NotFound().json("Not found 404");
}

fn init_auth_routes(
    auth_controller: Data<AuthController>,
    container: Arc<Container>
) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = ()
    >
> {
    return web
        ::scope("/auth")
        .app_data(auth_controller.clone())
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .service(protected_route(container, "").route("/logout", web::post().to(logout)));
}

fn init_user_routes(
    us_controller: Data<UserController>,
    container: Arc<Container>
) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = ()
    >
> {
    return protected_route(container, "/user")
        .app_data(us_controller)
        .route("/all", web::get().to(find_all))
        .route("", web::get().to(find_me))
        .route("", web::delete().to(delete));
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
