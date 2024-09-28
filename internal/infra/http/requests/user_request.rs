use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
