use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[derive(Clone)]
pub struct Auth {
    pub fingerprint: String,
    pub fighter: Option<Fighter>,
}

#[derive(Clone)]
pub struct Fighter {
    pub emo: String,
    pub rank: i32,
}

#[derive(Debug)]
pub enum AuthError {
    MissingFingerprint,
    InternalServerError(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Accept fingerprint from header or ?fp= query param (browsers can't set headers on WS)
        let fingerprint = req
            .headers()
            .get_one(ebobo_shared::AUTH_HEADER)
            .map(|s| s.to_string())
            .or_else(|| {
                req.uri()
                    .query()
                    .and_then(|q| {
                        q.to_string().split('&').find_map(|pair| {
                            let (k, v) = pair.split_once('=')?;
                            if k == "fp" { Some(v.to_owned()) } else { None }
                        })
                    })
            });

        match fingerprint {
            None => request::Outcome::Error((Status::Unauthorized, AuthError::MissingFingerprint)),
            Some(fp) => match req.rocket().state::<crate::EboboState>() {
                None => request::Outcome::Error((
                    Status::InternalServerError,
                    AuthError::InternalServerError("missing application state".to_string()),
                )),
                Some(state) => {
                    let fighter = match crate::entities::prelude::Fighters::find()
                        .filter(crate::entities::fighters::Column::Fingerprint.eq(fp.clone()))
                        .one(state.db.as_ref())
                        .await
                    {
                        Ok(f) => f.map(|f| Fighter { emo: f.emo, rank: f.rank }),
                        Err(e) => return request::Outcome::Error((
                            Status::InternalServerError,
                            AuthError::InternalServerError(e.to_string()),
                        )),
                    };
                    request::Outcome::Success(Auth { fingerprint: fp, fighter })
                }
            },
        }
    }
}
