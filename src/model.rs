use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTicketModel {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(ModelController {
            tickets_store: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ticket: CreateTicketModel) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let id = tickets_store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket.title,
        };
        tickets_store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let tickets_store = self.tickets_store.lock().unwrap();
        Ok(tickets_store.iter().filter_map(|t| t.clone()).collect())
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store.get_mut(id as usize).ok_or(Error::TicketDeleteFailIdNotFound { id })?;
        let ticket = ticket.take().ok_or(Error::TicketDeleteFailIdNotFound { id })?;
        Ok(ticket)
    }
}