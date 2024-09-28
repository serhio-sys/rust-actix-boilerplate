use std::sync::{ Arc, Mutex };

use actix_web::{ web, HttpResponse, Responder };

use crate::{
    infra::{
        domain::session::SessionDTO,
        http::requests::user_request::{ AuthRequest, UserRequest },
    },
    services::auth_service::AuthService,
};

#[derive(Clone)]
pub struct AuthController {
    auth_service: Arc<Mutex<AuthService>>,
}

impl AuthController {
    pub fn new(auth_service: Arc<Mutex<AuthService>>) -> AuthController {
        return AuthController { auth_service };
    }

    async fn register(&self, user: web::Json<UserRequest>) -> impl Responder {
        match self.auth_service.lock().unwrap().register(user.into_inner()) {
            Ok(user) => {
                return HttpResponse::Created().json(user);
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(e.to_string());
            }
        }
    }

    async fn login(&self, user_credentials: web::Json<AuthRequest>) -> impl Responder {
        match self.auth_service.lock().unwrap().login(user_credentials.into_inner()) {
            Ok(user) => {
                return HttpResponse::Ok().json(user);
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(e.to_string());
            }
        }
    }

    async fn logout(&self, session: SessionDTO) {
        todo!();
    }
}

pub async fn register(
    auth_controller: web::Data<AuthController>,
    user: web::Json<UserRequest>
) -> impl Responder {
    return auth_controller.register(user).await;
}

pub async fn login(
    auth_controller: web::Data<AuthController>,
    user: web::Json<AuthRequest>
) -> impl Responder {
    return auth_controller.login(user).await;
}
