use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom};

use crate::{repositories::UserRepository, auth};

use super::{DbConn, server_error};

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<auth::Credentials>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
            .map(|user| {
                if let Ok(token) = auth::authorize_user(&user, &credentials) {
                    return json!(token)
                }
                json!("Unauthorized")
            })
            .map_err(|e| server_error(e.into()))
    }).await
}