use actix_web::{delete, put, post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
mod game_card_model;
use crate::game_card_model::GameCard;

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

// Route pour récupérer toute les cartes
#[get("/gameCards")]
async fn get_all_game_cards(data: web::Data<Arc<Mutex<AppState>>>) -> HttpResponse {
    let state = data.lock().unwrap();
    HttpResponse::Ok().json(&state.game_cards)
}

// Route pour créer un carte
#[post("/gameCards")]
async fn create_game_card(data: web::Data<Arc<Mutex<AppState>>>, new_game_card: web::Json<GameCard>) -> impl Responder {
    let mut state = data.lock().unwrap();
    let mut local_new_game_card = new_game_card.into_inner();
    local_new_game_card.set_id(Some(state.generate_unique_id().try_into().unwrap()));
    state.game_cards.push(local_new_game_card);
    HttpResponse::Created().json(state.game_cards.last().unwrap())
}

// Route pour modifier une carte
#[put("/gameCards/{game_card_id}")]
async fn update_card(data: web::Data<Arc<Mutex<AppState>>>, path: web::Path<i32>, updated_game_card: web::Json<GameCard>) -> HttpResponse {
    let game_card_id = path.0;
    let mut state = data.lock().unwrap();
   // print(game_card.get_id());
    if let Some(index) = state.game_cards.iter().position(|game_card| game_card.get_id() == Some(game_card_id)) {
        if let Some(game_card) = state.game_cards.get_mut(index) {
            let updated_data = updated_game_card.into_inner();
            game_card.set_outcome(updated_data.get_outcome());
            game_card.set_game_type(updated_data.get_game_type());
            game_card.set_character(updated_data.get_character());
            game_card.set_kda(updated_data.get_kda());
            game_card.set_role(updated_data.get_role());
            game_card.set_comment(updated_data.get_comment());
            HttpResponse::Ok().json(game_card)
        } else {
            HttpResponse::InternalServerError().body(format!("Failed to update the card"))
        }
    } else {
        HttpResponse::NotFound().body(format!("Carte non trouvée"))
    }
}

#[delete("/gameCards/{game_card_id}")]
async fn delete_card(data: web::Data<Arc<Mutex<AppState>>>, path: web::Path<i32>) -> HttpResponse {
    let game_card_id = path.0;
    let mut state = data.lock().unwrap();
    if let Some(index) = state.game_cards.iter().position(|card| card.get_id() == Some(game_card_id)) {
        state.game_cards.remove(index);
        HttpResponse::Ok().body(format!("Carte supprimée"))
    } else {
        HttpResponse::NotFound().body(format!("Carte non trouvée"))
    }
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
            .service(get_all_game_cards)
            .service(create_game_card)
            .service(update_card)
            .service(delete_card)
    })
    .bind("127.0.0.1:4242")?
    .run()
    .await
}