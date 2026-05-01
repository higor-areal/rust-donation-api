use std::{net::ToSocketAddrs, sync::{Arc, Mutex}};
use axum::{self, routing::get};

struct Donation{
    donor: String,
    amount: f64,
}

struct AppState{
    total: f64,
    donations: Vec<Donation>,
}


#[tokio::main]
async fn main() {

    let mut state = AppState{
        total: 0.0,
        donations: vec![]
    };

    let shared = std::sync::Arc::new(std::sync::Mutex::new(state));

    let app = axum::Router::new()
    .route("/", get(home))
    .route("/total", get(get_total));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> String{
    "Sejá bem vindo a API do higor".to_string()
}

async fn get_total() -> String{
    "Total = 0".to_string()
}