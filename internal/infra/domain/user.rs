use std::sync::Arc;

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::infra::{
    database::user_repository::User,
    http::{ middlewares::Userable, resources::user_resource::UserResponse },
};

#[derive(Clone, PartialEq, Serialize)]
pub struct UserDTO {
    pub id: Arc<Option<i32>>,
    pub name: Arc<str>,
    pub password: Arc<str>,
    pub email: Arc<str>,
    pub avatar: Option<Arc<str>>,
    pub created_date: Arc<NaiveDateTime>,
    pub updated_date: Arc<NaiveDateTime>,
    pub deleted_date: Arc<Option<NaiveDateTime>>,
}

#[derive(Clone, Serialize)]
pub struct AuthenticatedUserDTO {
    pub user: UserResponse,
    pub token: Arc<str>,
}

impl UserDTO {
    pub(crate) fn model_to_dto(user: User) -> UserDTO {
        UserDTO {
            id: Arc::new(Some(user.id)),
            name: Arc::from(user.name),
            password: Arc::from(user.password),
            email: Arc::from(user.email),
            avatar: user.avatar.map(Arc::from),
            created_date: Arc::new(user.created_date),
            updated_date: Arc::new(user.updated_date),
            deleted_date: Arc::new(user.deleted_date),
        }
    }

    pub(crate) fn models_to_dto(users: Vec<User>) -> Vec<UserDTO> {
        users.into_iter().map(UserDTO::model_to_dto).collect()
    }

    pub fn dto_to_model(&self) -> User {
        User {
            id: self.id
                .as_ref()
                .and_then(|id| Some(id))
                .unwrap(),
            name: self.name.to_string(),
            password: self.password.to_string(),
            email: self.email.to_string(),
            avatar: self.avatar.as_ref().map(|arc_str| arc_str.as_ref().to_string()),
            created_date: *self.created_date,
            updated_date: *self.updated_date,
            deleted_date: self.deleted_date.as_ref().and_then(|date| Some(date)),
        }
    }
}

impl Userable for UserDTO {
    fn get_user_id(&self) -> Arc<i32> {
        return Arc::from(self.id.clone().unwrap());
    }
}
