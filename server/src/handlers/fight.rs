use std::sync::Arc;
use std::time::Duration;
use futures::SinkExt;
use rocket::State;
use sea_orm::{prelude::*, *};
use ebobo_shared::Utc;

use crate::{
    entities::{fighters, matches, plays, prelude::*},
    EboboState,
};

#[options("/fight")]
pub async fn options() {}

#[get("/fight")]
pub async fn post(
    auth: crate::auth::Auth,
    ws: rocket_ws::WebSocket,
    state: &State<EboboState>,
) -> rocket_ws::Channel<'static> {
    let db = state.db.clone();
    let fingerprint = auth.fingerprint.clone();

    ws.channel(move |mut stream| {
        let db = db.clone();
        let fingerprint = fingerprint.clone();
        let auth = auth.clone();
        Box::pin(async move {
            // Mark this fighter as queued
            Fighters::update(fighters::ActiveModel {
                queued: ActiveValue::set(true),
                ..Default::default()
            })
            .filter(fighters::Column::Fingerprint.eq(fingerprint.clone()))
            .exec(db.as_ref())
            .await
            .unwrap();

            // Try to find an opponent
            let opponents = Fighters::find()
                .filter(
                    fighters::Column::Fingerprint
                        .ne(fingerprint.clone())
                        .and(fighters::Column::Queued.eq(true)),
                )
                .limit(1)
                .all(db.as_ref())
                .await
                .unwrap_or_default();

            if let Some(enemy) = opponents.into_iter().next() {
                let fighter = auth.fighter.clone().unwrap();

                let (mut enemy_r, mut my_r) = (enemy.rank + 1, fighter.rank + 1);
                let winner = if enemy.rank == fighter.rank {
                    enemy_r += 1;
                    my_r += 1;
                    None
                } else if enemy.rank < fighter.rank {
                    my_r += 2;
                    Some(fingerprint.clone())
                } else {
                    enemy_r += 2;
                    Some(enemy.fingerprint.clone())
                };

                // Update both fighters' ranks and clear queue status
                let my_fp = fingerprint.clone();
                let enemy_fp = enemy.fingerprint.clone();
                let db1 = Arc::clone(&db);
                let db2 = Arc::clone(&db);
                let (r1, r2) = futures::join!(
                    Fighters::update(fighters::ActiveModel {
                        rank: ActiveValue::set(my_r),
                        queued: ActiveValue::set(false),
                        ..Default::default()
                    })
                    .filter(fighters::Column::Fingerprint.eq(my_fp))
                    .exec(db1.as_ref()),
                    Fighters::update(fighters::ActiveModel {
                        rank: ActiveValue::set(enemy_r),
                        queued: ActiveValue::set(false),
                        ..Default::default()
                    })
                    .filter(fighters::Column::Fingerprint.eq(enemy_fp.clone()))
                    .exec(db2.as_ref())
                );
                r1.unwrap();
                r2.unwrap();

                // Record the match
                let match_id = Uuid::new_v4();
                Matches::insert(matches::ActiveModel {
                    id: ActiveValue::set(match_id),
                    winner: ActiveValue::set(winner.clone()),
                    date: ActiveValue::set(Utc::now().naive_utc()),
                })
                .exec(db.as_ref())
                .await
                .unwrap();

                let db3 = Arc::clone(&db);
                let db4 = Arc::clone(&db);
                let my_fp2 = fingerprint.clone();
                let (p1, p2) = futures::join!(
                    Plays::insert(plays::ActiveModel {
                        r#match: ActiveValue::set(match_id),
                        fighter: ActiveValue::set(my_fp2),
                        ..Default::default()
                    })
                    .exec(db3.as_ref()),
                    Plays::insert(plays::ActiveModel {
                        r#match: ActiveValue::set(match_id),
                        fighter: ActiveValue::set(enemy_fp),
                        ..Default::default()
                    })
                    .exec(db4.as_ref())
                );
                p1.ok();
                p2.ok();

                if let Some(winner_fp) = winner {
                    let _ = stream.send(rocket_ws::Message::Text(winner_fp)).await;
                }
            } else {
                tokio::time::sleep(Duration::from_secs(5)).await;
            }

            Ok(())
        })
    })
}
