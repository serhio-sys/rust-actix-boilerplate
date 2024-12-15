use core::error;
use std::{ sync::Arc, time::{ Duration, SystemTime, UNIX_EPOCH } };

use config::CONFIGURATION;
use jsonwebtoken::{ EncodingKey, Header };
use serde::{ Deserialize, Serialize };
use thiserror::Error;
use rust_commons::{ base64::{ self, Engine }, pwhash, uuid::Uuid };
use rust_commons::crypto::bcrypt::{ verify_password, hash_password };

use crate::{
    filesystem::image_storage_service::ImageStorageService,
    infra::{
        database::{
            session_repository::{ Session, SessionRepository },
            user_repository::UserRepository,
        },
        domain::{ session::SessionDTO, user::{ AuthenticatedUserDTO, UserDTO } },
        http::{
            requests::user_request::{ AuthRequest, UserRequest },
            resources::user_resource::UserResponse,
        },
    },
};

use super::user_image_name;

#[derive(Serialize, Clone, Deserialize)]
pub struct Claims {
    pub user_id: Arc<i32>,
    pub uuid: Uuid,
    pub exp: usize,
}

pub struct AuthService {
    user_repository: Arc<UserRepository>,
    session_repository: Arc<SessionRepository>,
    file_system: Arc<ImageStorageService>,
}

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("{0}")] DieselError(diesel::result::Error),
    #[error("{0}")] ArgonError(pwhash::error::Error),
    #[error("{0}")] JWTError(jsonwebtoken::errors::Error),
    #[error("{0}")] ServiceError(Box<dyn error::Error + Send + Sync + 'static>),
}

impl AuthService {
    pub fn new(
        user_repository: Arc<UserRepository>,
        session_repository: Arc<SessionRepository>,
        file_system: Arc<ImageStorageService>
    ) -> Arc<AuthService> {
        return Arc::new(AuthService {
            session_repository,
            user_repository,
            file_system,
        });
    }

    pub async fn register(
        &self,
        mut user: UserRequest
    ) -> Result<AuthenticatedUserDTO, AuthServiceError> {
        if self.user_repository.find_by_email(&user.email).is_ok() {
            return Err(
                AuthServiceError::ServiceError(
                    Box::from("User already exists with provided email!")
                )
            );
        }
        let hashed_password = hash_password(&user.password).map_err(AuthServiceError::ArgonError)?;
        user.password = hashed_password;

        if let Some(avatar_base64) = &user.avatar {
            if
                let Ok(decoded_image) =
                    base64::engine::general_purpose::STANDARD.decode(avatar_base64)
            {
                let user_image_path = user_image_name(&user.name);
                let new_filename = self.file_system
                    .save_image(&user_image_path, &decoded_image)
                    .map_err(AuthServiceError::ServiceError)?;
                user.avatar = Some(new_filename);
            }
        }

        let saved_user = self.user_repository
            .create_user(&user)
            .map_err(AuthServiceError::DieselError)?;

        let token = self.generate_jwt(Arc::from(saved_user.id))?;
        return Ok(AuthenticatedUserDTO {
            user: UserResponse::user_to_response(&saved_user),
            token: Arc::from(token),
        });
    }

    pub fn login(
        &self,
        request_user: AuthRequest
    ) -> Result<AuthenticatedUserDTO, AuthServiceError> {
        let user = self.user_repository
            .find_by_email(&request_user.email)
            .map_err(AuthServiceError::DieselError)?;

        if verify_password(&user.password, &request_user.password) {
            let user_dto = UserDTO::model_to_dto(user);
            let token = self.generate_jwt(Arc::new(user_dto.id.unwrap()))?;
            return Ok(AuthenticatedUserDTO {
                user: UserResponse::dto_to_response(&user_dto),
                token: Arc::from(token.to_string()),
            });
        }

        return Err(AuthServiceError::ServiceError(Box::from("Invalid password")));
    }

    pub fn logout(&self, session: SessionDTO) -> Result<(), AuthServiceError> {
        self.session_repository.delete(session).map_err(AuthServiceError::DieselError)?;
        return Ok(());
    }

    pub fn check(&self, session: Claims) -> bool {
        if
            self.session_repository
                .exists(SessionDTO {
                    user_id: session.user_id,
                    uuid: session.uuid,
                })
                .is_ok()
        {
            return true;
        }
        return false;
    }

    fn generate_jwt(&self, user_id: Arc<i32>) -> Result<String, AuthServiceError> {
        let session = SessionDTO { user_id, uuid: Uuid::new_v4() };
        let saved_session: Session = self.session_repository
            .save(session)
            .map_err(AuthServiceError::DieselError)?;
        let claims = Claims {
            user_id: saved_session.user_id.into(),
            uuid: saved_session.uuid,
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize) +
            (Duration::from_secs(CONFIGURATION.jwt_ttl).as_secs() as usize),
        };
        let token = jsonwebtoken
            ::encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(CONFIGURATION.jwt_secret.as_ref())
            )
            .map_err(AuthServiceError::JWTError)?;
        return Ok(token);
    }
}
