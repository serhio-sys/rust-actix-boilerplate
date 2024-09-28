use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SessionDTO {
    pub user_id: i32,
    pub uuid: Uuid,
}

impl SessionDTO {
    pub fn new(user_id: i32, uuid: Uuid) -> SessionDTO {
        return SessionDTO { user_id: user_id, uuid: uuid };
    }
}
