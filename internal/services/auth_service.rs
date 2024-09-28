use core::error;
use std::{ sync::{ Arc, Mutex }, time::{ Duration, SystemTime, UNIX_EPOCH } };

use argon2::{ Argon2, PasswordHash, PasswordVerifier };
use config::CONFIGURATION;
use dependencies::log::error;
use jsonwebtoken::{ EncodingKey, Header };
use serde::{ Deserialize, Serialize };
use thiserror::Error;
use uuid::Uuid;

use crate::infra::{
    database::{
        session_repository::{ Session, SessionRepository },
        user_repository::UserRepository,
    },
    domain::{ session::SessionDTO, user::{ RegisteredUserDTO, UserDTO } },
    http::requests::user_request::{ AuthRequest, UserRequest },
};

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: i32,
    uuid: Uuid,
    exp: usize,
}

pub struct AuthService {
    user_repository: Arc<Mutex<UserRepository>>,
    session_repository: Arc<Mutex<SessionRepository>>,
}

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("Database error: {0}")] DieselError(diesel::result::Error),
    #[error("Hash error: {0}")] ArgonError(argon2::password_hash::Error),
    #[error("JWT error: {0}")] JWTError(jsonwebtoken::errors::Error),
    #[error("Service error: {0}")] ServiceError(Box<dyn error::Error>),
}

impl AuthService {
    pub fn new(
        user_repository: Arc<Mutex<UserRepository>>,
        session_repository: Arc<Mutex<SessionRepository>>
    ) -> Arc<Mutex<AuthService>> {
        return Arc::new(
            Mutex::new(AuthService {
                session_repository,
                user_repository,
            })
        );
    }

    pub fn register(&mut self, user: UserRequest) -> Result<RegisteredUserDTO, AuthServiceError> {
        if let Ok(_) = self.user_repository.lock().unwrap().find_by_email(&user.email) {
            return Err(
                AuthServiceError::ServiceError(
                    Box::from("User is already by provided email exists!")
                )
            );
        }
        let user_dto: UserDTO;
        match self.user_repository.lock().unwrap().create_user(&user) {
            Ok(saved_user) => {
                user_dto = UserDTO::model_to_dto(saved_user);
            }
            Err(e) => {
                return Err(AuthServiceError::DieselError(e));
            }
        }
        match self.generate_jwt(user_dto.id.unwrap()) {
            Ok(token) => {
                return Ok(RegisteredUserDTO { user: user_dto, token: token });
            }
            Err(e) => Err(e),
        }
    }

    pub fn login(&mut self, request_user: AuthRequest) -> Result<UserDTO, AuthServiceError> {
        match self.user_repository.lock().unwrap().find_by_email(&request_user.email) {
            Ok(user) => {
                if verify_password(&user.password, &request_user.password) {
                    return Ok(UserDTO::model_to_dto(user));
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

    fn generate_jwt(&mut self, user_id: i32) -> Result<String, AuthServiceError> {
        let session = SessionDTO { user_id: user_id, uuid: Uuid::new_v4() };
        let saved_session: Session;
        match self.session_repository.lock().unwrap().save(session) {
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
    let parsed_hash = PasswordHash::new(hash);
    if let Err(_) = parsed_hash {
        return false;
    }
    let argon2 = Argon2::default();
    if let Ok(_) = argon2.verify_password(password.as_bytes(), &parsed_hash.unwrap()) {
        return true;
    }
    return false;
}
