use std::ops::Deref;

use diesel::prelude::*;
use tide::{error::ResultExt, response, Context, EndpointResult};

use crate::{
    db::user_model::{CreateUser, User},
    AppData,
};

pub async fn create(mut cx: Context<AppData>) -> EndpointResult {
    use crate::schema::users::{self as user, dsl::*};
    let data: CreateUser = cx.body_json().await.client_err()?;
    let db = cx.app_data().db();
    let conn = db.deref();
    diesel::insert_into(user::table)
        .values(&data)
        .execute(conn)
        .server_err()?;
    let u = users
        .order(id.desc())
        .limit(1)
        .get_result::<User>(conn)
        .server_err()?;
    Ok(response::json(u))
}
