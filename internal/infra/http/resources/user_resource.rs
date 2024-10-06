use chrono::NaiveDateTime;
use serde::Serialize;

use crate::infra::domain::user::UserDTO;

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>,
}

impl UserResponse {
    pub fn dto_to_response(dto: &UserDTO) -> Self {
        return UserResponse {
            id: dto.id.unwrap(),
            name: dto.name.clone(),
            email: dto.email.clone(),
            created_date: dto.created_date,
            updated_date: dto.updated_date,
            deleted_date: dto.deleted_date,
        };
    }

    pub fn dtos_to_response(dtos: Vec<UserDTO>) -> Vec<Self> {
        let mut response_objects: Vec<Self> = Vec::new();
        for dto in dtos {
            response_objects.push(Self::dto_to_response(&dto));
        }
        return response_objects;
    }
}
