use crate::{ctx::Context, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub creator_id: u64,
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
    pub async fn create_ticket(&self, context: Context, ticket: CreateTicketModel) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let id = tickets_store.len() as u64;
        let ticket = Ticket {
            id,
            creator_id: context.user_id(),
            title: ticket.title,
        };
        tickets_store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _context: Context) -> Result<Vec<Ticket>> {
        let tickets_store = self.tickets_store.lock().unwrap();
        Ok(tickets_store.iter().filter_map(|t| t.clone()).collect())
    }

    pub async fn delete_ticket(&self, id: u64, _context: Context) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let ticket = tickets_store.get_mut(id as usize).ok_or(Error::TicketDeleteFailIdNotFound { id })?;
        let ticket = ticket.take().ok_or(Error::TicketDeleteFailIdNotFound { id })?;
        Ok(ticket)
    }
}