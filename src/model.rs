use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ticket {
    pub id: u64,
    pub title: String
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String
}
