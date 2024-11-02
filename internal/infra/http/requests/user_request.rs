use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserRequest {
    #[validate(length(min = 4, message = "Name must be at least 4 characters long"))]
    pub name: String,
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub password: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserUpdateRequest {
    #[validate(length(min = 4, message = "Name must be at least 4 characters long"))]
    pub name: Option<String>,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthRequest {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 4, message = "Password must be at least 4 characters long"))]
    pub password: String,
}
