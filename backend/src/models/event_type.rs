use crate::schema::event_types;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;

#[table_name = "event_types"]
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
pub struct Eventtype {
    pub event_type_id: i8,
    pub name: String,
    pub description: String,
    pub default_points: f32,
}

impl Eventtype {
    pub fn read_all(connection: &MysqlConnection) -> Vec<Eventtype> {
        event_types::table.order(event_types::event_type_id.asc()).load::<Eventtype>(connection).unwrap()
    }

    pub fn get_by_id(id: i8, connection: &MysqlConnection) -> Option<Eventtype> {
        let statement = event_types::table.filter(event_types::event_type_id.eq(&id));
        let event_type = statement.load::<Eventtype>(connection);
        match event_type {
            Ok(mut event_type) => event_type.pop(),
            Err(_) => None,
        }
    }

    pub fn validate_event_type_id(event_type_id: i8, connection: &MysqlConnection) -> (bool, Option<Eventtype>) {
        let result = Eventtype::get_by_id(event_type_id, &connection);
        (result.is_some(), result)
    }
}