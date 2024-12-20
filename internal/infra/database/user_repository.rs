use std::sync::{ Arc, RwLock };

use chrono::{ NaiveDateTime, Utc };
use rust_commons::diesel::{
    prelude::{ AsChangeset, Insertable, Queryable },
    query_dsl::methods::FilterDsl,
    r2d2::{ ConnectionManager, Pool, PooledConnection },
    ExpressionMethods,
    PgConnection,
    RunQueryDsl,
    Selectable,
    SelectableHelper,
};

use crate::infra::{ domain::user::UserDTO, http::requests::user_request::UserRequest };

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        avatar -> Nullable<Text>,
        created_date -> Timestamp,
        updated_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
    }
}

#[derive(Selectable, AsChangeset, Queryable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
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
    pub(crate) avatar: Option<String>,
    pub(crate) created_date: Option<NaiveDateTime>,
    pub(crate) updated_date: Option<NaiveDateTime>,
    pub(crate) deleted_date: Option<NaiveDateTime>,
}

impl UserInsertable {
    fn new(
        name: String,
        user_email: String,
        user_password: String,
        avatar: Option<String>
    ) -> UserInsertable {
        return UserInsertable {
            name: name,
            email: user_email,
            password: user_password,
            avatar: avatar,
            created_date: None,
            updated_date: None,
            deleted_date: None,
        };
    }
}

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>,
}

impl UserRepository {
    pub fn new(pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>) -> Arc<UserRepository> {
        return Arc::new(UserRepository { pool });
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.write().unwrap().get().unwrap()
    }

    pub fn create_user(&self, user_dto: &UserRequest) -> Result<User, diesel::result::Error> {
        use users::dsl::users;
        let user_model = UserInsertable::new(
            user_dto.name.clone(),
            user_dto.email.clone(),
            user_dto.password.clone(),
            user_dto.avatar.clone()
        );
        let new_user = diesel
            ::insert_into(users)
            .values(&user_model)
            .returning(User::as_returning())
            .get_result(&mut self.get_connection())?;
        return Ok(new_user);
    }

    pub fn find_all(&self) -> Result<Vec<User>, diesel::result::Error> {
        use self::users::dsl::{ users, deleted_date };
        let users_list = users
            .filter(deleted_date.is_null())
            .load::<User>(&mut self.get_connection())?;
        return Ok(users_list);
    }

    pub fn find_by_id(&self, user_id: Arc<i32>) -> Result<User, diesel::result::Error> {
        use self::users::dsl::*;
        return users
            .filter(id.eq(*user_id))
            .filter(deleted_date.is_null())
            .first::<User>(&mut self.get_connection())
            .map_err(Into::into);
    }

    pub fn find_by_email(&self, user_email: &str) -> Result<User, diesel::result::Error> {
        use self::users::dsl::*;
        return users
            .filter(email.eq(user_email))
            .filter(deleted_date.is_null())
            .first::<User>(&mut self.get_connection())
            .map_err(Into::into);
    }

    pub fn update(&self, user_to_update: &UserDTO) -> Result<User, diesel::result::Error> {
        use self::users::dsl::*;
        let user = user_to_update.dto_to_model();
        let query = diesel::update(users.filter(id.eq(user.id)));
        return query
            .set(user)
            .returning(User::as_returning())
            .get_result(&mut self.get_connection());
    }

    pub fn update_avatar(
        &self,
        user_id: Arc<i32>,
        file_name: String
    ) -> Result<User, diesel::result::Error> {
        use self::users::dsl::*;
        let query = diesel::update(users.filter(id.eq(*user_id)));
        return query
            .set(avatar.eq(file_name))
            .returning(User::as_returning())
            .get_result(&mut self.get_connection());
    }

    pub fn delete(&self, user_id: Arc<i32>) -> Result<usize, diesel::result::Error> {
        use self::users::dsl::*;
        return diesel
            ::update(users)
            .filter(id.eq(*user_id))
            .set(deleted_date.eq(Utc::now().naive_local()))
            .execute(&mut self.get_connection());
    }
}
