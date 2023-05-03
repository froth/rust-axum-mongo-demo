use demo_web_app_lib::persistence::ticket::MongoTicketRepository;
use demo_web_app_lib::web::{routes_login, routes_tickets};
use demo_web_app_lib::error::Result;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let client_options = ClientOptions::parse(
        "mongodb://localhost:27017/?connectTimeoutMS=200&serverSelectionTimeoutMS=200",
    )
    .await
    .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let mongoc = MongoTicketRepository { client };

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_login::routes())
        .nest("/api", routes_tickets::routes(Arc::new(mongoc)))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - ", "RESP_MAPPER");
    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong> {name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong> {name}</strong>"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
