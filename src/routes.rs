use crate::server::handle_query_param;
use crate::web::login::api_login;
use crate::web::ticket_routes;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router {
    let route_hello = Router::new().route("/hello", get(handle_query_param));

    route_hello
}

pub fn login_routes() -> Router {
    let route_login = Router::new().route("/api/login", post(api_login));

    route_login
}
