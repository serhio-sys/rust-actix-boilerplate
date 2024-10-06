use core::error;
use std::sync::Arc;
use config::log::error;
use thiserror::Error;

use crate::infra::{
    database::{ session_repository::SessionRepository, user_repository::UserRepository },
    domain::user::UserDTO,
    http::{ middlewares::Findable, requests::{ user_request::UserUpdateRequest, JsonValidator } },
};

pub struct UserService {
    session_repository: Arc<SessionRepository>,
    user_repository: Arc<UserRepository>,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Database error: {0}")] DieselError(diesel::result::Error),
    #[error("{0}")] ServiceError(Box<dyn error::Error + Send + Sync + 'static>),
}

impl Findable<UserDTO> for UserService {
    fn find_by_id(
        &self,
        user_id: i32
    ) -> Result<UserDTO, Box<dyn std::error::Error + Send + Sync + 'static>> {
        return Ok(self.find_by_id(user_id)?);
    }
}

impl UserService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        session_repository: Arc<SessionRepository>
    ) -> Arc<UserService> {
        return Arc::from(UserService {
            user_repository: user_repo,
            session_repository: session_repository,
        });
    }

    pub fn find_all(&self) -> Result<Vec<UserDTO>, diesel::result::Error> {
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

    pub fn find_by_id(&self, user_id: i32) -> Result<UserDTO, UserServiceError> {
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

    pub fn update(
        &self,
        current_user: &mut UserDTO,
        update_data: JsonValidator<UserUpdateRequest>
    ) -> Result<UserDTO, UserServiceError> {
        if let Some(name) = &update_data.name {
            current_user.name = name.to_string();
        }
        if let Some(email) = &update_data.email {
            if let Ok(_) = self.user_repository.find_by_email(&email) {
                return Err(
                    UserServiceError::ServiceError(
                        Box::from("User is already exists by provided email!")
                    )
                );
            }
            current_user.email = email.to_string();
        }
        match self.user_repository.update(current_user) {
            Ok(user) => {
                return Ok(UserDTO::model_to_dto(user));
            }
            Err(e) => {
                error!("Error in User Service: {}", e);
                return Err(UserServiceError::DieselError(e));
            }
        }
    }

    pub fn delete(&self, user_id: i32) -> Result<(), UserServiceError> {
        match self.user_repository.delete(user_id) {
            Ok(_) => {
                match self.session_repository.delete_by_user_id(user_id) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(UserServiceError::DieselError(e));
                    }
                }
            }
            Err(e) => {
                return Err(UserServiceError::DieselError(e));
            }
        }
    }
}
