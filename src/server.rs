use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Default, Debug)]
pub struct Server;

#[derive(Debug, Deserialize)]
pub struct HelloParam {
    name: Option<String>,
}

impl Server {}

pub async fn handle_query_param(Query(params): Query<HelloParam>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("Hell");

    println!("Query params: {:?} {:?}", params, name);
    // Html(format!("Hello, {}!", name))

    Html(format!("Hello, {name}"))
}
