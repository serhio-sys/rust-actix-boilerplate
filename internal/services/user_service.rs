use core::error;
use std::sync::Arc;
use config::log::error;
use thiserror::Error;

use crate::{
    filesystem::image_storage_service::ImageStorageService,
    infra::{
        database::{ session_repository::SessionRepository, user_repository::UserRepository },
        domain::user::UserDTO,
        http::{
            middlewares::Findable,
            requests::{ user_request::UserUpdateRequest, JsonValidator },
        },
    },
};

pub struct UserService {
    session_repository: Arc<SessionRepository>,
    user_repository: Arc<UserRepository>,
    file_system: Arc<ImageStorageService>,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Database error: {0}")] DieselError(diesel::result::Error),
    #[error("{0}")] ServiceError(Box<dyn error::Error + Send + Sync + 'static>),
}

impl Findable<UserDTO> for UserService {
    fn find_by_id(
        &self,
        user_id: Arc<i32>
    ) -> Result<UserDTO, Box<dyn std::error::Error + Send + Sync + 'static>> {
        return Ok(self.find_by_id(user_id)?);
    }
}

impl UserService {
    pub fn new(
        user_repository: Arc<UserRepository>,
        session_repository: Arc<SessionRepository>,
        file_system: Arc<ImageStorageService>
    ) -> Arc<UserService> {
        return Arc::from(UserService {
            user_repository,
            session_repository,
            file_system,
        });
    }

    pub fn find_all(&self) -> Result<Vec<UserDTO>, diesel::result::Error> {
        let users = self.user_repository.find_all()?;
        return Ok(UserDTO::models_to_dto(users));
    }

    pub fn find_by_id(&self, user_id: Arc<i32>) -> Result<UserDTO, diesel::result::Error> {
        let user = self.user_repository.find_by_id(user_id)?;
        return Ok(UserDTO::model_to_dto(user));
    }

    pub fn update(
        &self,
        current_user: &mut UserDTO,
        update_data: JsonValidator<UserUpdateRequest>
    ) -> Result<UserDTO, UserServiceError> {
        if let Some(name) = &update_data.name {
            current_user.name = Arc::from(name.to_string());
        }
        if let Some(email) = &update_data.email {
            if self.user_repository.find_by_email(&email).is_ok() {
                return Err(
                    UserServiceError::ServiceError(
                        Box::from("User is already exists by provided email!")
                    )
                );
            }
            current_user.email = Arc::from(email.to_string());
        }
        let user = self.user_repository
            .update(current_user)
            .map_err(UserServiceError::DieselError)?;

        return Ok(UserDTO::model_to_dto(user));
    }

    pub fn delete(
        &self,
        user: &UserDTO
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let user_id_ref = Arc::new(user.id.unwrap());
        self.user_repository.delete(user_id_ref.clone())?;
        self.session_repository.delete_by_user_id(user_id_ref)?;
        if let Some(avatar) = &user.avatar {
            self.file_system.remove_file_image(avatar)?;
        }
        return Ok(());
    }
}
