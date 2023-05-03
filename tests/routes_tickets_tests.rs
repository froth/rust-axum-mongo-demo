use std::sync::{Arc, Mutex};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use demo_web_app_lib::{
    error::{Error, Result},
    model::{Ticket, TicketForCreate},
    persistence::ticket::TicketRepository,
    web::routes_tickets,
};
use tower::ServiceExt;
use async_trait::async_trait;

struct FakeRepo {
    tickets_store: Mutex<Vec<Option<Ticket>>>,
}

impl FakeRepo {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Mutex::default(),
        })
    }
}

#[async_trait]
impl TicketRepository for FakeRepo {
    async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        Ok(store.iter().filter_map(|x| x.clone()).collect())
    }

    async fn delete(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let element = store.get_mut(id as usize).and_then(|elem| elem.take());

        element.ok_or(Error::IdNotFound { id })
    }
}
#[tokio::test]
async fn list_tickets() {
    let fake_repo = FakeRepo::new().await.unwrap();
    fake_repo.create_ticket(TicketForCreate { title: "foo".to_string() }).await.unwrap();
    let route = routes_tickets::routes(Arc::new(fake_repo));
    let response = route
        .oneshot(Request::builder().uri("/tickets").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Vec<Ticket> = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, vec!(Ticket{id: 0, title: "foo".to_string()}))
}
