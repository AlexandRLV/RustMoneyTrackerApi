use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, Error, Result};

#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("--> {:<12} - api_login - {payload:?}", "HANDLER");

    if payload.username != "username" || payload.password != "password" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}