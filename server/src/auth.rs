use crate::*;
use rocket::response::status::BadRequest;
use std::net::SocketAddr;

#[post("/", data = "<fingerprint>")]
pub fn auth(
    fingerprint: &str,
    state: &State<AuthState>,
    remote_addr: SocketAddr,
) -> Result<String, BadRequest<String>> {
    let fingerprint = Fingerprint {
        value: fingerprint.to_string(),
        address: remote_addr
    };

    /* 
    let _ = state
        .persist
        .save::<Fingerprint>(
            format!("fingerprint_{}", &fingerprint.value.as_str()).as_str(),
            fingerprint.clone(),
        )
        .map_err(|e| BadRequest(Some(e.to_string())));
    */

    if fingerprint.value == "🐱" {
        unimplemented!();
    }
    
    let greet = "Hello, 🐱!";
    
    Ok(greet.to_owned())
}
