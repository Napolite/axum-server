use axum::Router;
use tower_cookies::CookieManagerLayer;
mod error;
mod routes;
mod server;
mod web;

pub use core::error::Error;

use routes::{login_routes, routes};

#[tokio::main]
async fn main() {
    // let server = Server::default();

    let route_all = Router::new()
        .merge(routes())
        .merge(login_routes())
        .layer(CookieManagerLayer::new());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(route_all.into_make_service())
        .await
        .unwrap();
    println!("Server running at http://{}", addr);
}
