use chrono::NaiveDateTime;
use serde::Serialize;

use crate::infra::database::user_repository::User;

#[derive(Clone, PartialEq, Serialize)]
pub struct UserDTO {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>,
}

#[derive(Clone, PartialEq, Serialize)]
pub struct RegisteredUserDTO {
    pub user: UserDTO,
    pub token: String,
}

impl UserDTO {
    pub(crate) fn model_to_dto(user: User) -> UserDTO {
        return UserDTO {
            id: Some(user.id),
            name: user.name,
            email: user.email,
            password: user.password,
            created_date: user.created_date,
            updated_date: user.updated_date,
            deleted_date: user.deleted_date,
        };
    }

    pub(crate) fn models_to_dto(users: Vec<User>) -> Vec<UserDTO> {
        let mut users_dto: Vec<UserDTO> = Vec::new();
        for user in users {
            users_dto.push(UserDTO::model_to_dto(user));
        }
        return users_dto;
    }
}
