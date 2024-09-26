use std::sync::{ Arc, Mutex };

use dependencies::log::error;

use crate::infra::{ database::user_repository::UserRepository, domain::user::UserDTO };

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Arc<Mutex<UserService>> {
        return Arc::new(Mutex::new(UserService { user_repository: user_repo }));
    }

    pub fn get_all_users(&mut self) -> Vec<UserDTO> {
        match self.user_repository.find_all() {
            Ok(users) => {
                return UserDTO::models_to_dto(users);
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Vec::new();
            }
        }
    }
}
