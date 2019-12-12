use crate::schema::passwords;

#[table_name = "passwords"]
#[derive(Serialize, Queryable, Insertable, AsChangeset)]
pub struct Password {
    pub password_id: Option<i32>,
    pub password: String,
    pub verification_code: String,
}