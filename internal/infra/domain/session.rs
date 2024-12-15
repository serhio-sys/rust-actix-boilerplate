use std::sync::Arc;

use serde::{ Deserialize, Serialize };
use rust_commons::uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SessionDTO {
    pub user_id: Arc<i32>,
    pub uuid: Uuid,
}

impl SessionDTO {
    pub fn new(user_id: Arc<i32>, uuid: Uuid) -> SessionDTO {
        return SessionDTO { user_id, uuid };
    }
}
