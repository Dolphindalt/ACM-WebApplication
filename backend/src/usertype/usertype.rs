use crate::schema::user_types;

#[table_name = "user_types"]
#[derive(Serialize, Queryable, Insertable, AsChangeset)]
pub struct Usertype {
    pub user_type_id: Option<i8>,
    pub name: String,
    pub description: String,
}