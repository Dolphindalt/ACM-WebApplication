use crate::schema::users;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub user_id: Option<i32>,
    pub password_id: Option<i32>,
    pub user_type: i8,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");
        users::table.order(users::user_id.desc()).first(connection).unwrap()
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<User> {
        users::table.order(users::user_id.asc()).load::<User>(connection).unwrap()
    }

    pub fn update(user_id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(user_id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(user_id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(user_id)).execute(connection).is_ok()
    }
}