use std::time::Duration;

use rocket::{futures::*, State};
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
    Fighters::update(fighters::ActiveModel {
        queued: ActiveValue::set(true),
        ..Default::default()
    })
    .filter(fighters::Column::Fingerprint.eq(auth.fingerprint.clone()))
    .exec(state.db.as_ref())
    .await
    .unwrap();

    let db = state.db.clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let queue = Fighters::find()
                    .filter(
                        fighters::Column::Fingerprint
                            .ne(auth.fingerprint.clone())
                            .and(fighters::Column::Queued.eq(true)),
                    )
                    .limit(1)
                    .all(db.as_ref())
                    .await;

                if let Ok(e) = queue {
                    if !e.is_empty() {
                        let e = e.first().unwrap();
                        let fighter = auth.fighter.unwrap();

                        let mut enemy_r = e.rank + 1;
                        let mut my_r = fighter.rank + 1;

                        let winner = if e.rank == fighter.rank {
                            enemy_r += 1;
                            my_r += 1;
                            None
                        } else if e.rank < fighter.rank {
                            my_r += 2;
                            Some(auth.fingerprint.clone())
                        } else {
                            enemy_r += 2;
                            Some(e.fingerprint.clone())
                        };

                        // TODO: tx

                        Fighters::update(fighters::ActiveModel {
                            rank: ActiveValue::set(my_r),
                            queued: ActiveValue::set(false),
                            ..Default::default()
                        })
                        .filter(fighters::Column::Fingerprint.eq(auth.fingerprint.clone()))
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        Fighters::update(fighters::ActiveModel {
                            rank: ActiveValue::set(enemy_r),
                            queued: ActiveValue::set(false),
                            ..Default::default()
                        })
                        .filter(fighters::Column::Fingerprint.eq(e.fingerprint.clone()))
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        let id = Uuid::new_v4();

                        Matches::insert(matches::ActiveModel {
                            id: ActiveValue::set(id),
                            winner: ActiveValue::set(winner.clone()),
                            date: ActiveValue::set(Utc::now().naive_utc()),
                        });

                        Plays::insert(plays::ActiveModel {
                            r#match: ActiveValue::set(id),
                            fighter: ActiveValue::set(auth.fingerprint.clone()),
                            ..Default::default()
                        })
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        Plays::insert(plays::ActiveModel {
                            r#match: ActiveValue::set(id),
                            fighter: ActiveValue::set(e.fingerprint.clone()),
                            ..Default::default()
                        })
                        .exec(db.as_ref())
                        .await
                        .unwrap();

                        let _ = stream.send(rocket_ws::Message::Text(winner.unwrap())).await; // TODO: fix

                        break;
                    }
                }

                std::thread::sleep(Duration::from_secs(5));
            }

            Ok(())
        })
    })
}
