use std::sync::Arc;

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::infra::{ database::user_repository::User, domain::user::UserDTO };

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: Arc<i32>,
    pub name: Arc<str>,
    pub email: Arc<str>,
    pub avatar: Option<Arc<str>>,
    pub created_date: Arc<NaiveDateTime>,
    pub updated_date: Arc<NaiveDateTime>,
    pub deleted_date: Arc<Option<NaiveDateTime>>,
}

impl UserResponse {
    pub fn dto_to_response(dto: &UserDTO) -> Self {
        return UserResponse {
            id: Arc::new(dto.id.unwrap()),
            name: dto.name.clone(),
            email: dto.email.clone(),
            avatar: dto.avatar.clone(),
            created_date: dto.created_date.clone(),
            updated_date: dto.updated_date.clone(),
            deleted_date: dto.deleted_date.clone(),
        };
    }

    pub fn user_to_response(dto: &User) -> Self {
        return UserResponse {
            id: Arc::from(dto.id),
            name: Arc::from(dto.name.to_owned()),
            email: Arc::from(dto.email.to_owned()),
            avatar: dto.avatar.to_owned().map(Arc::from),
            created_date: Arc::new(dto.created_date),
            updated_date: Arc::new(dto.updated_date),
            deleted_date: Arc::new(dto.deleted_date),
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
