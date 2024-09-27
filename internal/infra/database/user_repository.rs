use chrono::{ NaiveDateTime, Utc };
use diesel::{
    prelude::{ Insertable, Queryable },
    query_dsl::methods::FilterDsl,
    r2d2::{ ConnectionManager, Pool, PooledConnection },
    ExpressionMethods,
    PgConnection,
    RunQueryDsl,
    Selectable,
    SelectableHelper,
};

use crate::infra::domain::user::UserDTO;

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
    }
}

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
        use users::dsl::users;
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
        use users::dsl::users;
        let users_list = users.load::<User>(&mut self.get_connection())?;
        return Ok(users_list);
    }

    pub fn find_by_id(&mut self, user_id: i32) {
        use users::dsl::users;
    }
}
