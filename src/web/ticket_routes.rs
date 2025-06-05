use crate::Result;
use crate::{ModelController, Ticket, TicketForCrate};
use axum::extract::{FromRef, Path};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCrate>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    let ticket = mc.delete_tickets(id).await?;

    Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Routes {
    let app_state = AppState(mc);
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/ticket/:id", delete(delete_tickets))
        .with_state(app_state)
}
