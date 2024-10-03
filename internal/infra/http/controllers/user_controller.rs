use std::sync::Arc;

use actix_web::{ web, HttpMessage, HttpRequest, HttpResponse, Responder };

use crate::{
    infra::{ domain::user::UserDTO, http::resources::BasedListResponse },
    services::user_service::UserService,
};

#[derive(Clone)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> UserController {
        return UserController { user_service };
    }

    async fn find_all(&self) -> impl Responder {
        match self.user_service.find_all() {
            Ok(users) => {
                let response = BasedListResponse {
                    data: users,
                    total: 0,
                    page: 0,
                };
                return HttpResponse::Ok().json(response);
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(e.to_string());
            }
        }
    }

    async fn find_me(&self, request: HttpRequest) -> impl Responder {
        if let Some(user) = request.extensions_mut().get::<UserDTO>() {
            return HttpResponse::Ok().json(user);
        }
        return HttpResponse::BadRequest().json("Something went wrong");
    }

    async fn delete(&self, request: HttpRequest) -> impl Responder {
        if let Some(user) = request.extensions_mut().get::<UserDTO>() {
            match self.user_service.delete(user.id.unwrap()) {
                Ok(_) => {
                    return HttpResponse::Ok().finish().map_into_boxed_body();
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(e.to_string());
                }
            }
        }
        return HttpResponse::Forbidden().json("Not authenticated");
    }
}

// HANDLERS USER ROUTE
pub async fn find_all(user_controller: web::Data<UserController>) -> impl Responder {
    return user_controller.find_all().await;
}

pub async fn find_me(
    user_controller: web::Data<UserController>,
    request: HttpRequest
) -> impl Responder {
    return user_controller.find_me(request).await;
}

pub async fn delete(
    user_controller: web::Data<UserController>,
    request: HttpRequest
) -> impl Responder {
    return user_controller.delete(request).await;
}
