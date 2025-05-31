use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCrate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::Default(),
        })
    }

    pub async fn create_tickets(&self, ticket_fc: TicketForCrate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;

        let ticket = Ticket{
            id:id,
            title ticket_fc.title
        }

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }
}
