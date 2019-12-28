use crate::schema::event_files;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::models::file::File;

#[table_name = "event_files"]
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
pub struct Eventfile {
    pub dummy_id: Option<i32>,
    pub file_id: i32,
    pub event_id: i32,
    pub additional_info: Option<String>,
}

impl Eventfile {
    pub fn get_by_event(event_id: i32, connecton: &MysqlConnection) -> Vec<Eventfile> {
        let statement = event_files::table.filter(event_files::event_id.eq(&event_id));
        let file = statement.load::<Eventfile>(connecton);
        match file {
            Ok(file) => file,
            Err(_) => Vec::new(),
        }
    }

    pub fn get_file(&self, connection: &MysqlConnection) -> File {
        File::get_by_id(self.event_id, connection).unwrap()
    }
}