use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use crate::models::event_type::Eventtype;
use crate::models::event::Event;
use crate::models::event_file::Eventfile;
use crate::models::file::File;
use crate::models::user::User;
use chrono::NaiveDateTime;
use crate::auth::APIKey;
use rocket::response::status::{Custom};

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

#[post("/create", data = "<event_medium>")]
fn create(event_medium: Json<NewEventMedium>, key: APIKey, connection: Connection) -> Result<Json<Event>, Custom<String>> {
    let request_user: User = match crate::auth::get_user_from_token_string(key.0) {
        Some(ru) => ru,
        None => return Err(Custom(Status::Unauthorized, String::from("You do not have access to this resource.")))
    };
    if !User::is_admin(request_user.user_type, &connection) {
        return Err(Custom(Status::Unauthorized, String::from("You do not have access to this resource.")));
    }
    let mut medium = event_medium.into_inner();
    let (val_user, _) = User::validate_user_id(medium.coordinator_id, &connection);
    let (val_event_type, event_type) = Eventtype::validate_event_type_id(medium.event_type_id, &connection);
    if val_user && val_event_type {
        if medium.points < 0.0 {
            medium.points = event_type.unwrap().default_points;
        }
        let event = Event {
            event_id: None,
            coordinator_id: Some(medium.coordinator_id),
            event_type_id: medium.event_type_id,
            name: medium.name,
            additional_info: Some(medium.additional_info),
            location: medium.location,
            event_time: medium.event_time,
            points: medium.points,
        };
        let event = Event::create(event, &connection);
        Ok(Json(event))
    } else {
        Err(Custom(Status::BadRequest, String::from("Invalid user or event type")))
    }
}

#[get("/")]
pub fn get_all(connection: Connection) -> Result<Json<JsonValue>, Custom<String>> {
    let events: Vec<Event> = Event::read_all(&connection);
    let events_model: Vec<EventModel> = events
        .into_iter()
        .map(|event| {
            let event_id: i32 = event.event_id.unwrap();
            let event_files = Eventfile::get_by_event(event_id, &connection);
            let files: Vec<FileModel> = event_files
                .into_iter()
                .map(|event_file| {
                    let file = event_file.get_file(&connection);
                    FileModel {
                        event_file,
                        file,
                    }
                })
                .collect();
            EventModel { 
                event: event, 
                files: files,
            }
        })
        .collect();
    Ok(Json(json!{events_model}))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/event", routes![create, get_all])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::Event;
    use chrono::NaiveDateTime;
    use std::str::FromStr;
    use crate::init_rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use crate::endpoints::test::admin_authorization_header;
    use crate::endpoints::test::default_authorization_header;

    #[test]
    fn create_event_test_good() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/event", routes![super::create]);
        let client = Client::new(rocket).unwrap();
        let body_event: Event = Event {
            event_id: None,
            coordinator_id: Some(1),
            event_type_id: 1,
            name: String::from("Test event"),
            additional_info: Some(String::from("This is a test event")),
            location: String::from("Dalton\'s house"),
            event_time: NaiveDateTime::from_str("2007-04-05T14:30:30").ok().unwrap(),
            points: 0.0,
        };
        let mut request = client.post("/event/create");
        request.add_header(admin_authorization_header());
        let mut response = request
            .body(serde_json::to_string::<Event>(&body_event).ok().unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let event_result: Event = serde_json::from_str::<Event>(response.body_string().unwrap().as_str()).ok().unwrap();
        assert_eq!(event_result.coordinator_id, body_event.coordinator_id);
        assert_eq!(event_result.event_type_id, body_event.event_type_id);
        assert_eq!(event_result.name, body_event.name);
        assert_eq!(event_result.location, body_event.location);
        assert_eq!(event_result.event_time, body_event.event_time);
    }

    #[test]
    fn create_event_test_bad_authentication() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/event", routes![super::create]);
        let client = Client::new(rocket).unwrap();
        let body_event: Event = Event {
            event_id: None,
            coordinator_id: Some(1),
            event_type_id: 1,
            name: String::from("Test event"),
            additional_info: Some(String::from("This is a test event")),
            location: String::from("Dalton\'s house"),
            event_time: NaiveDateTime::from_str("2007-04-05T14:30:30").ok().unwrap(),
            points: 0.0,
        };
        let mut request = client.post("/event/create");
        request.add_header(default_authorization_header());
        let response = request
            .body(serde_json::to_string::<Event>(&body_event).ok().unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn get_all_events_test_good() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/event", routes![super::get_all]);
        let client = Client::new(rocket).unwrap();
        let request = client.get("/event").dispatch();
        assert_eq!(request.status(), Status::Ok);
    }

}