use crate::schema::files;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;

#[table_name = "files"]
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
pub struct File {
    pub file_id: Option<i32>,
    pub uploader: i32,
    pub audience: i8,
    pub file_name: String,
    pub description: String,
}

impl File {
    pub fn get_by_uploader(uploader: i32, connecton: &MysqlConnection) -> Vec<File> {
        let statement = files::table.filter(files::uploader.eq(&uploader));
        let file = statement.load::<File>(connecton);
        match file {
            Ok(file) => file,
            Err(_) => Vec::new(),
        }
    }

    pub fn get_by_id(file_id: i32, connection: &MysqlConnection) -> Option<File> {
        let statement = files::table.filter(files::file_id.eq(&file_id));
        let file = statement.load::<File>(connection);
        match file {
            Ok(mut file) => file.pop(),
            Err(_) => None,
        }
    }
}