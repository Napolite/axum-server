use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketForCrate};
use axum::{
    Router,
    extract::{FromRef, Json, Path, State},
    routing::{delete, get, post},
};
use std::io::{stdout, Write};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCrate>,
) -> Result<Json<Ticket>> {

    println!("--> This route was hit completely");
    let ticket = mc.create_ticket(ticket_fc).await?;

    println!("this route was hit");

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;

    println!("this route was hit");
    stdout().flush().unwrap();

    Ok(Json(tickets))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    let ticket = mc.delete_tickets(id).await?;

    Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/ticket/:id", delete(delete_tickets))
        .with_state(app_state)
}
