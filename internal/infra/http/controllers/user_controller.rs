use std::sync::{ Arc, Mutex };

use actix_web::{ HttpResponse, Responder };

use crate::{ infra::http::resources::BasedResponse, service::user_service::UserService };

#[derive(Clone)]
pub struct UserController {
    user_service: Arc<Mutex<UserService>>,
}

impl UserController {
    pub fn new(user_service: Arc<Mutex<UserService>>) -> UserController {
        return UserController { user_service };
    }

    pub async fn get_users(&self) -> impl Responder {
        let users = self.user_service.lock().unwrap().get_all_users();
        let response = BasedResponse {
            data: users,
            total: 0,
            page: 0,
        };
        return HttpResponse::Ok().json(response);
    }
}
