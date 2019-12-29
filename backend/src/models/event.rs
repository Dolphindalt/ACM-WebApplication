use crate::schema::events;
use diesel::mysql::MysqlConnection;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use chrono::NaiveDateTime;
use diesel::dsl;
use crate::models::event_file::Eventfile;
use crate::models::file::File;

#[table_name = "events"]
#[derive(Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
pub struct Event {
    pub event_id: Option<i32>,
    pub coordinator_id: Option<i32>,
    pub event_type_id: i8,
    pub name: String,
    pub additional_info: Option<String>,
    pub location: String,
    pub event_time: NaiveDateTime,
    pub points: f32,
}

#[derive(Serialize, Deserialize)]
struct NewEventMedium {
    pub coordinator_id: i32,
    pub event_type_id: i8,
    pub name: String,
    pub additional_info: String,
    pub location: String,
    pub event_time: NaiveDateTime,
    pub points: f32,
}

#[derive(Serialize)]
pub struct FileModel {
    pub event_file: Eventfile,
    pub file: File,
}

#[derive(Serialize)]
pub struct EventModel {
    pub event: Event,
    pub files: Vec<FileModel>,
}

impl Event {
    pub fn create(event: Event, connection: &MysqlConnection) -> Event {
        diesel::insert_into(events::table)
            .values(&event)
            .execute(connection)
            .expect("Error creating new event");
        events::table.order(events::event_id.desc()).first(connection).unwrap()
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<Event> {
        events::table.order(events::event_id.asc()).load::<Event>(connection).unwrap()
    }

    pub fn read_all_upcoming_events(connection: &MysqlConnection) -> Vec<Event> {
        events::table.filter(events::event_time.gt(dsl::now)).load::<Event>(connection).unwrap()
    }

    pub fn read_all_past_events(connection: &MysqlConnection) -> Vec<Event> {
        events::table.filter(events::event_time.lt(dsl::now)).load::<Event>(connection).unwrap()
    }

    pub fn update(event_id: i32, event: Event, connection: &MysqlConnection) -> bool {
        diesel::update(events::table.find(event_id)).set(&event).execute(connection).is_ok()
    }

    pub fn delete(event_id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(events::table.find(event_id)).execute(connection).is_ok()
    }
}

impl EventModel {
    pub fn get_events(collector: &dyn Fn(&MysqlConnection) -> Vec<Event>, connection: &MysqlConnection) -> Vec<EventModel> {
        let events = collector(connection);
        let events_model: Vec<EventModel> = events
            .into_iter()
            .map(|event| {
                let event_id: i32 = event.event_id.unwrap();
                let event_files = Eventfile::get_by_event(event_id, &connection);
                let files: Vec<FileModel> = event_files
                    .into_iter()
                    .filter(|event_file| {
                        event_file.get_file(&connection).is_some()
                    })
                    .map(|event_file| {
                        let file = event_file.get_file(&connection).unwrap();
                        FileModel {
                            event_file: event_file,
                            file: file,
                        }
                    })
                    .collect();
                EventModel { 
                    event: event, 
                    files: files,
                }
            })
            .collect();
        events_model
    }
}