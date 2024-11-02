use core::error;
use std::{ sync::Arc, time::{ Duration, SystemTime, UNIX_EPOCH } };

use base64::Engine;
use config::CONFIGURATION;
use config::log::warn;
use pwhash::bcrypt::{ self, BcryptSetup };
use jsonwebtoken::{ EncodingKey, Header };
use serde::{ Deserialize, Serialize };
use thiserror::Error;
use uuid::Uuid;

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

#[derive(Serialize, Clone, Deserialize)]
pub struct Claims {
    pub user_id: i32,
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
        if let Ok(_) = self.user_repository.find_by_email(&user.email) {
            return Err(
                AuthServiceError::ServiceError(
                    Box::from("User is already exists by provided email!")
                )
            );
        }
        let hash_result = hash_user_password(&user.password);
        if let Ok(hashed) = hash_result {
            user.password = hashed;
            match self.user_repository.create_user(&user) {
                Ok(saved_user) => {
                    let mut user_to_response = saved_user;
                    let filename = format!("user/user_avatar_{}.png", user_to_response.id);
                    if let Some(avatar) = user.avatar {
                        if
                            let Ok(decoded_image) =
                                base64::engine::general_purpose::STANDARD.decode(avatar)
                        {
                            if let Err(e) = self.file_system.save_image(&filename, &decoded_image) {
                                let _ = self.user_repository.delete(user_to_response.id);
                                return Err(AuthServiceError::ServiceError(e));
                            }
                            match self.user_repository.update_avatar(user_to_response.id, filename) {
                                Ok(updated_user) => {
                                    user_to_response = updated_user;
                                }
                                Err(e) => {
                                    let _ = self.user_repository.delete(user_to_response.id);
                                    return Err(AuthServiceError::DieselError(e));
                                }
                            }
                        }
                    }
                    match self.generate_jwt(user_to_response.id) {
                        Ok(token) => {
                            return Ok(AuthenticatedUserDTO {
                                user: UserResponse::user_to_response(&user_to_response),
                                token,
                            });
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                }
                Err(e) => {
                    return Err(AuthServiceError::DieselError(e));
                }
            }
        }
        return Err(AuthServiceError::ArgonError(hash_result.unwrap_err()));
    }

    pub fn login(
        &self,
        request_user: AuthRequest
    ) -> Result<AuthenticatedUserDTO, AuthServiceError> {
        match self.user_repository.find_by_email(&request_user.email) {
            Ok(user) => {
                if verify_password(&user.password, &request_user.password) {
                    let user_dto = UserDTO::model_to_dto(user);
                    match self.generate_jwt(user_dto.id.unwrap()) {
                        Ok(token) => {
                            return Ok(AuthenticatedUserDTO {
                                user: UserResponse::dto_to_response(&user_dto),
                                token: token.to_string(),
                            });
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    return Err(AuthServiceError::ServiceError(Box::from("Invalid password")));
                }
            }
            Err(_) => {
                return Err(
                    AuthServiceError::ServiceError(
                        Box::from("User was not found by provided email")
                    )
                );
            }
        }
    }

    pub fn logout(&self, session: SessionDTO) -> Result<(), AuthServiceError> {
        match self.session_repository.delete(session) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(AuthServiceError::DieselError(e));
            }
        }
    }

    pub fn check(&self, session: Claims) -> bool {
        match
            self.session_repository.exists(SessionDTO {
                user_id: session.user_id,
                uuid: session.uuid,
            })
        {
            Ok(exists) => {
                if exists {
                    return true;
                }
            }
            Err(e) => {
                warn!("{}", e.to_string());
            }
        }
        return false;
    }

    fn generate_jwt(&self, user_id: i32) -> Result<String, AuthServiceError> {
        let session = SessionDTO { user_id, uuid: Uuid::new_v4() };
        let saved_session: Session;
        match self.session_repository.save(session) {
            Ok(unwrapped_session) => {
                saved_session = unwrapped_session;
            }
            Err(e) => {
                return Err(AuthServiceError::DieselError(e));
            }
        }
        let claims = Claims {
            user_id: saved_session.user_id,
            uuid: saved_session.uuid,
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize) +
            (Duration::from_secs(CONFIGURATION.jwt_ttl).as_secs() as usize),
        };
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(CONFIGURATION.jwt_secret.as_ref())
        );
        match token {
            Ok(token_str) => {
                return Ok(token_str);
            }
            Err(e) => {
                return Err(AuthServiceError::JWTError(e));
            }
        }
    }
}

fn verify_password(hash: &str, password: &str) -> bool {
    return bcrypt::verify(password, hash);
}

pub fn hash_user_password(password: &str) -> Result<String, pwhash::error::Error> {
    return bcrypt::hash_with(
        BcryptSetup {
            variant: Some(bcrypt::BcryptVariant::V2b),
            cost: Some(5),
            ..Default::default()
        },
        password
    );
}
