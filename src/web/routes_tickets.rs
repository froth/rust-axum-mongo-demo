use axum::Router;
use axum::extract::Path;
use axum::routing::{delete, post};
use axum::{extract::State, Json};

use crate::model::{Ticket, TicketForCreate};
use crate::error::Result;
use crate::persistence::ticket::DynTicketRepository;

async fn create_ticket(State(mc): State<DynTicketRepository>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<DynTicketRepository>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(State(mc): State<DynTicketRepository>, Path(id): Path<u64>) ->  Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete(id).await?;

    Ok(Json(ticket))
}

pub fn routes(mc: DynTicketRepository) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket)).with_state(mc)
}