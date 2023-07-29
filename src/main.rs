use actix_web::{delete, put, post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
mod game_card_model;
use crate::game_card_model::GameCard;
use crate::game_card_model::Role;

// Initalisation du state
#[derive(Clone, Serialize, Deserialize)]
struct AppState {
    game_cards: Vec<GameCard>,
    counter: u64,
}

impl AppState {
    // Fonction pour générer un nouvel ID de carte unique
    fn generate_unique_id(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }
}

// Route pour créer un carte
#[post("/gameCard")]
async fn create_game_card(data: web::Data<Arc<Mutex<AppState>>>, new_game_card: web::Json<GameCard>) -> impl Responder {
    let mut state = data.lock().unwrap();
    let mut local_new_game_card = new_game_card.into_inner();
    local_new_game_card.set_id(Some(state.generate_unique_id().try_into().unwrap()));
    state.game_cards.push(local_new_game_card);
    HttpResponse::Created().json(state.game_cards.last().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialisation de mon "state"
    let initial_state = AppState {
        game_cards: Vec::new(),
        counter: 0
    };
    let app_state = Arc::new(Mutex::new(initial_state));
    // Lancement du serveur HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(create_game_card)
    })
    .bind("127.0.0.1:4242")?
    .run()
    .await
}