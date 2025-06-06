use crate::error::{Error, Result};
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
            tickets_store: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn create_ticket(&self, ticket_fc: TicketForCrate) -> Result<Ticket> {

        println!("this route was hit sha");
        let mut store = self.tickets_store.lock().map_err(|_| Error::TicketCannotBeCreated)?;
        let id = store.len() as u64;

        let ticket = Ticket {
            id: id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().map_err(|_| Error::TicketCannotBeCreated)?;

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_tickets(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailedIdNotFound { id: id })
    }
}
