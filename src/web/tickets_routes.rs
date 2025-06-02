use crate::Result;
use crate::{ModelController, Ticket, TicketForCrate};

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCrate>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}
