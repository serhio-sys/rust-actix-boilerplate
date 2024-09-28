use std::sync::{ Arc, Mutex };

use actix_web::{ web, HttpResponse, Responder };

use crate::{ infra::http::resources::BasedListResponse, services::user_service::UserService };

#[derive(Clone)]
pub struct UserController {
    user_service: Arc<Mutex<UserService>>,
}

impl UserController {
    pub fn new(user_service: Arc<Mutex<UserService>>) -> UserController {
        return UserController { user_service };
    }

    async fn get_users(&self) -> impl Responder {
        match self.user_service.lock().unwrap().get_all_users() {
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

    async fn get_user_by_id(&self, user_id: i32) -> impl Responder {
        match self.user_service.lock().unwrap().get_user_by_id(user_id) {
            Ok(user) => {
                return HttpResponse::Ok().json(user);
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(e.to_string());
            }
        }
    }
}

// HANDLERS USER ROUTE
pub async fn get_users(user_controller: web::Data<UserController>) -> impl Responder {
    return user_controller.get_users().await;
}

pub async fn get_user_by_id(
    user_controller: web::Data<UserController>,
    user_id: web::Path<i32>
) -> impl Responder {
    return user_controller.get_user_by_id(user_id.into_inner()).await;
}
