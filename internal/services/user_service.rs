use std::sync::{ Arc, Mutex };

use argon2::{ password_hash::SaltString, Argon2, PasswordHasher };
use dependencies::log::error;
use thiserror::Error;

use crate::infra::{
    database::user_repository::UserRepository,
    domain::user::UserDTO,
    http::requests::user_request::UserRequest,
};

pub struct UserService {
    user_repository: Arc<Mutex<UserRepository>>,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Database error: {0}")] DieselError(diesel::result::Error),
    #[error("Hash error: {0}")] ArgonError(argon2::password_hash::Error),
}

impl UserService {
    pub fn new(user_repo: Arc<Mutex<UserRepository>>) -> Arc<Mutex<UserService>> {
        return Arc::new(Mutex::new(UserService { user_repository: user_repo }));
    }

    pub fn get_all_users(&mut self) -> Result<Vec<UserDTO>, diesel::result::Error> {
        match self.user_repository.lock().unwrap().find_all() {
            Ok(users) => {
                return Ok(UserDTO::models_to_dto(users));
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(e);
            }
        }
    }

    pub fn get_user_by_id(&mut self, user_id: i32) -> Result<UserDTO, UserServiceError> {
        match self.user_repository.lock().unwrap().find_by_id(user_id) {
            Ok(user) => {
                return Ok(UserDTO::model_to_dto(user));
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(UserServiceError::DieselError(e));
            }
        }
    }

    pub fn create_user(&mut self, user: &mut UserRequest) -> Result<UserDTO, UserServiceError> {
        match hash_user_password(&user.password) {
            Ok(password) => {
                user.password = password;
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(UserServiceError::ArgonError(e));
            }
        }
        match self.user_repository.lock().unwrap().create_user(&user) {
            Ok(created) => {
                return Ok(UserDTO::model_to_dto(created));
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(UserServiceError::DieselError(e));
            }
        };
    }
}

fn hash_user_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}
