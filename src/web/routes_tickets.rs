use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::ctx::Context;
use crate::model::{ModelController, Ticket, CreateTicketModel};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(context: Context, State(mc): State<ModelController>, Json(create_model): Json<CreateTicketModel>) -> Result<Json<Ticket>> {
    println!("--> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(context, create_model).await?;
    Ok(Json(ticket))
}

async fn list_tickets(context: Context, State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("--> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(context).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(context: Context, State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
    println!("--> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(id, context).await?;
    Ok(Json(ticket))
}