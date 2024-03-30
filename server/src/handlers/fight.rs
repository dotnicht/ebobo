use rocket::{response::status::BadRequest, State};
use sea_orm::*;

use crate::{
    entities::{prelude::*, users::*},
    guards::auth::Auth,
    AppState,
};

#[options("/choose")]
pub async fn options() {}

#[post("/choose", data = "<request>")]
pub async fn choose(
    auth: Auth,
    request: String,
    state: &State<AppState>,
) -> Result<(), BadRequest<String>> {
    let count = Users::find()
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to fetch users count: {}", e)))?
        .into_iter()
        .count();

    let user = ActiveModel {
        id: Default::default(),
        fingerprint: ActiveValue::set(auth.fingerprint.clone()),
        rank: Default::default(),
        root: ActiveValue::set(count == 0),
        fighter: ActiveValue::set(request),
    };

    Users::insert(user)
        .exec(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(format!("Failed to insert user: {}", e.to_string())))?;

    Ok(())
}
