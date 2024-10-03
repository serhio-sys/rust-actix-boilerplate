use std::sync::Arc;
use dependencies::log::error;
use thiserror::Error;

use crate::infra::{ database::user_repository::UserRepository, domain::user::UserDTO };

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Database error: {0}")] DieselError(diesel::result::Error),
}

impl UserService {
    pub fn new(user_repo: Arc<UserRepository>) -> Arc<UserService> {
        return Arc::from(UserService { user_repository: user_repo });
    }

    pub fn get_all_users(&self) -> Result<Vec<UserDTO>, diesel::result::Error> {
        match self.user_repository.find_all() {
            Ok(users) => {
                return Ok(UserDTO::models_to_dto(users));
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(e);
            }
        }
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Result<UserDTO, UserServiceError> {
        match self.user_repository.find_by_id(user_id) {
            Ok(user) => {
                return Ok(UserDTO::model_to_dto(user));
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(UserServiceError::DieselError(e));
            }
        }
    }
}
