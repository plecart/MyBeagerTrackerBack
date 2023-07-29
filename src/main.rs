use actix_web::{delete, put, post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
mod gameCardModel;
use crate::gameCardModel::GameCard;
use crate::gameCardModel::Role;

// Initalisation du state
#[derive(Clone, Serialize, Deserialize)]
struct AppState {
    gameCards: Vec<GameCard>,
    counter: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialisation de mon "state"
    let initial_state = AppState {
        gameCards: Vec::new(),
        counter: 0
    };
    let app_state = Arc::new(Mutex::new(initial_state));
    // Lancement du serveur HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
    })
    .bind("127.0.0.1:4242")?
    .run()
    .await
}