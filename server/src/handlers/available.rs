use rocket::{response::status::BadRequest, serde::json::Json, State};
use sea_orm::*;

use ebobo_shared::Fighter;

use crate::{
    entities::{fighters, prelude::*},
    EboboState,
};

#[options("/available")]
pub async fn options() {}

#[get("/available")]
pub async fn get(state: &State<EboboState>) -> Result<Json<Vec<Fighter>>, BadRequest<String>> {
    let fighters = ['🐱', '🐵', '🐶', '🐷', '🐰', '🐮', '🐻'];

    let taken = Fighters::find()
        .column(fighters::Column::Emo)
        .all(state.db.as_ref())
        .await
        .map_err(|e| BadRequest(e.to_string()))?
        .into_iter()
        .map(|f| f.emo)
        .collect::<Vec<String>>();

    Ok(Json(
        fighters
            .into_iter()
            .filter(|f| !taken.contains(&f.to_string()))
            .map(|f| Fighter(f.to_string()))
            .collect(),
    ))
}
