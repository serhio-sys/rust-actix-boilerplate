use chrono::{ NaiveDateTime, Utc };
use diesel::{
    prelude::{ Insertable, Queryable },
    r2d2::{ ConnectionManager, Pool, PooledConnection },
    PgConnection,
    RunQueryDsl,
    Selectable,
    SelectableHelper,
};

use crate::infra::{ database::schemas::users::users, domain::user::UserDTO };

#[derive(Selectable, Queryable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone, Queryable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct UserInsertable {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) created_date: NaiveDateTime,
    pub(crate) updated_date: NaiveDateTime,
    pub(crate) deleted_date: Option<NaiveDateTime>,
}

impl UserInsertable {
    fn new(name: String, user_email: String, user_password: String) -> UserInsertable {
        return UserInsertable {
            name: name,
            email: user_email,
            password: user_password,
            created_date: Utc::now().naive_local(),
            updated_date: Utc::now().naive_local(),
            deleted_date: None,
        };
    }
}

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepository {
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get a connection")
    }

    pub fn create_user(&mut self, user_dto: &UserDTO) -> UserDTO {
        use super::schemas::users::users::dsl::*;
        let user_model = UserInsertable::new(
            user_dto.name.clone(),
            user_dto.email.clone(),
            user_dto.password.clone()
        );
        let new_user = diesel
            ::insert_into(users)
            .values(&user_model)
            .returning(User::as_returning())
            .get_result(&mut self.get_connection())
            .expect("Error in saving user");
        return UserDTO::model_to_dto(new_user);
    }

    pub fn find_all(&mut self) -> Result<Vec<User>, diesel::result::Error> {
        use super::schemas::users::users::dsl::*;
        let users_list = users.load::<User>(&mut self.get_connection())?;
        return Ok(users_list);
    }
}
