use axum::{middleware,Router};
use tower_cookies::CookieManagerLayer;
mod error;
mod model;
mod routes;
mod server;
mod web;

pub use core::error::Error;

use model::ModelController;
use routes::{login_routes, routes};

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let mc = ModelController::new().await?;
    // let server = Server::default();
    //
    let api_routes = web::ticket_routes::routes(mc.clone()).route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let route_all = Router::new()
        .merge(routes())
        .merge(login_routes())
        .nest("/api",api_routes )
        .layer(CookieManagerLayer::new());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(route_all.into_make_service())
        .await
        .unwrap();
    println!("Server running at http://{}", addr);

    Ok(())
}
