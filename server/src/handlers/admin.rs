use crate::domain::Device;
use crate::entities::{devices::*, prelude::*};
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use sea_query::*;
use std::sync::Arc;

#[get("/admin")]
pub async fn index(
    state: &State<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<Device>>, BadRequest<String>> {
    let devices = Devices::find()
        .all(state.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|device| Device {
            fingerprint: device.fingerprint,
            fighter: None,
        })
        .collect::<Vec<Device>>();

    Ok(Json(devices))
}

#[post("/admin/reset")]
pub async fn reset(state: &State<Arc<DatabaseConnection>>) -> Result<(), BadRequest<String>> {


    Ok(())
}
