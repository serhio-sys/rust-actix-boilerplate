pub mod user_service;
pub mod auth_service;

pub fn user_image_name(username: &str) -> String {
    return format!("users/user_{}.png", username);
}
