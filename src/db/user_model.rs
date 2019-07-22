use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}
