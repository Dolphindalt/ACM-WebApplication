use crate::schema::user_profiles;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::models::file::{File, FileModel};

#[table_name = "user_profiles"]
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct UserProfile {
    pub user_profile_id: Option<i32>,
    pub file_id: i32,
    pub user_id: i32,
}

impl UserProfile {
    pub fn get_by_user_id(user_id: i32, connection: &MysqlConnection) -> UserProfile {
        let statement = user_profiles::table.filter(user_profiles::user_id.eq(user_id));
        let user = statement.load::<UserProfile>(connection);
        match user {
            Ok(mut user) => if user.len() != 0 {
                user.pop().unwrap()
             } else {
                UserProfile { user_profile_id: None, file_id: 1, user_id: user_id }
            },
            Err(_) => UserProfile { user_profile_id: None, file_id: 1, user_id: user_id },
        }
    }

    pub fn into_file_model(&self, connection: &MysqlConnection) -> FileModel {
        if let Some(file) = File::get_by_id(self.file_id, connection) {
            file.into_file_model(connection)
        } else {
            // the default profile picture will be file 1
            File::get_by_id(1,  &connection).unwrap().into_file_model(connection)
        }
    }
}