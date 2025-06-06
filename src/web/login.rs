use crate::{
    error::{Error, Result},
    web,
};
use serde::Deserialize;
use serde_json::{Value, json};

use axum::Json;
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "admin" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    println!("Login successful: {:?}", payload);

    cookies.add(Cookie::new(web::AUTH_COOKIES, "user.sign.q"));
    let body = Json(json!({
        "result":{
            "success" : true
        }
    }));
    println!("Login successful: {:?}", cookies);
    Ok(body)
}
