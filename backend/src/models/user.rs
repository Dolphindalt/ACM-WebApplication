use crate::schema::users;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::models::user_type::Usertype;
use crate::models::file::FileModel;
use crate::models::user_profile::UserProfile;

#[table_name = "users"]
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub user_id: Option<i32>,
    pub password_id: Option<i32>,
    pub user_type: i8,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub points: f32,
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");
        users::table.order(users::user_id.desc()).first(connection).unwrap()
    }

    pub fn get_by_id(id: i32, connection: &MysqlConnection) -> Option<User> {
        let statement = users::table.filter(users::user_id.eq(id));
        let user = statement.load::<User>(connection);
        match user {
            Ok(mut user) => user.pop(),
            Err(_) => None,
        }
    }

    pub fn get_user_by_email(email: &String, connection: &MysqlConnection) -> Option<User> {
        let statement = users::table.filter(users::email.eq(&email));
        let user = statement.load::<User>(connection);
        match user {
            Ok(mut user) => user.pop(),
            Err(_) => None,
        }
    }

    pub fn get_user_by_email_and_password(email: &String, raw_password: &String, connection: &MysqlConnection) -> Option<User> {
        let user = match User::get_user_by_email(&email, &connection) {
            Some(user) => user,
            None => return None,
        };
        let password_id: i32 = user.password_id.unwrap();
        let password = crate::models::password::Password::get_by_password_id(password_id, &connection).unwrap();
        if password.password == crate::models::seed_new_password(raw_password.to_string()) {
            Some(user)
        } else {
            None
        }
    }

    pub fn validate_user_id(user_id: i32, connection: &MysqlConnection) -> (bool, Option<User>) {
        let result = User::get_by_id(user_id, &connection);
        (result.is_some(), result)
    }

    pub fn is_admin(user_type_id: i8, connection: &MysqlConnection) -> bool { 
        if let Some(user_type) = Usertype::read(user_type_id, &connection) {
            return user_type.is_admin;
        };
        false
    }

    pub fn get_officers(connection: &MysqlConnection) -> Vec<User> {
        users::table
            .filter(users::user_type.eq_any(vec![2, 3, 4, 5]))
            .load(connection).unwrap()
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

    pub fn into_user_models(collector: &dyn Fn(&MysqlConnection) -> Vec<User>, connection: &MysqlConnection) -> Vec<UserModel> {
        collector(connection)
            .into_iter()
            .filter(|user| user.user_id.is_some())
            .map(|user| {
                let user_profile = UserProfile::get_by_user_id(user.user_id.unwrap(), &connection).into_file_model(&connection);
                let user_type = Usertype::read(user.user_type, &connection).unwrap();
                UserModel {
                    user_type: user.user_type,
                    user_type_string: user_type.name,
                    email: user.email,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    profile_picture: user_profile,
                }
            })
            .collect::<Vec<UserModel>>()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserModel {
    pub user_type: i8,
    pub user_type_string: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: FileModel,
}