use std::sync::Arc;

use actix_web::{ web, HttpMessage, HttpRequest, HttpResponse, Responder };

use crate::{
    infra::{
        domain::session::SessionDTO,
        http::{
            requests::{ user_request::{ AuthRequest, UserRequest }, JsonValidator },
            resources::ErrorResponse,
        },
    },
    services::auth_service::{ AuthService, Claims },
};

#[derive(Clone)]
pub struct AuthController {
    auth_service: Arc<AuthService>,
}

impl AuthController {
    pub fn new(auth_service: Arc<AuthService>) -> AuthController {
        return AuthController { auth_service };
    }

    async fn register(&self, user: JsonValidator<UserRequest>) -> impl Responder {
        match self.auth_service.register(user.into_inner()).await {
            Ok(user) => {
                return HttpResponse::Created().json(user);
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(
                    ErrorResponse::new_error(Some(e.to_string()))
                );
            }
        }
    }

    async fn login(&self, user_credentials: web::Json<AuthRequest>) -> impl Responder {
        match self.auth_service.login(user_credentials.into_inner()) {
            Ok(user) => {
                return HttpResponse::Ok().json(user);
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(
                    ErrorResponse::new_error(Some(e.to_string()))
                );
            }
        }
    }

    async fn logout(&self, request: HttpRequest) -> impl Responder {
        if let Some(claims) = request.extensions_mut().get::<Claims>() {
            let session = SessionDTO {
                user_id: claims.user_id.clone(),
                uuid: claims.uuid.clone(),
            };
            match self.auth_service.logout(session) {
                Ok(_) => {
                    return HttpResponse::Ok().finish().map_into_boxed_body();
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(
                        ErrorResponse::new_error(Some(e.to_string()))
                    );
                }
            }
        } else {
            return HttpResponse::Unauthorized().finish();
        }
    }
}

pub async fn logout(
    auth_controller: web::Data<AuthController>,
    request: HttpRequest
) -> impl Responder {
    return auth_controller.logout(request).await;
}

pub async fn register(
    auth_controller: web::Data<AuthController>,
    user: JsonValidator<UserRequest>
) -> impl Responder {
    return auth_controller.register(user).await;
}

pub async fn login(
    auth_controller: web::Data<AuthController>,
    user: web::Json<AuthRequest>
) -> impl Responder {
    return auth_controller.login(user).await;
}
