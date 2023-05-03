use std::sync::Arc;

use async_trait::async_trait;
use mongodb::{
    bson::doc,
    options::{CountOptions, FindOptions, InsertOneOptions, FindOneAndDeleteOptions},
    Client,
};
use crate::{
    model::{Ticket, TicketForCreate},
    error::{Result, Error::IdNotFound}
};

use futures::TryStreamExt;

pub type DynTicketRepository = Arc<dyn TicketRepository + Send + Sync>;

#[async_trait]
pub trait TicketRepository {
    async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket>;
    async fn list_tickets(&self) -> Result<Vec<Ticket>>;
    async fn delete(&self, id: u64) -> Result<Ticket>;
}
pub struct MongoTicketRepository {
    pub client: Client,
}

#[async_trait]
impl TicketRepository for MongoTicketRepository {
    async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let collection = self.client.database("db").collection::<Ticket>("c");

        let id = collection
            .count_documents(doc! {}, CountOptions::default())
            .await?;

        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        collection
            .insert_one(ticket.clone(), InsertOneOptions::default())
            .await?;

        Ok(ticket)
    }
    async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let collection = self.client.database("db").collection::<Ticket>("c");
        let tickets = collection
            .find(doc! {}, FindOptions::default())
            .await?
            .try_collect()
            .await?;

        Ok(tickets)
    }
    async fn delete(&self, id: u64) -> Result<Ticket> {
        let collection = self.client.database("db").collection::<Ticket>("c");
        let ticket = collection.find_one_and_delete(doc! {"id" : id as u32}, FindOneAndDeleteOptions::default()).await?;
        ticket.ok_or(IdNotFound{id})
    }
}

