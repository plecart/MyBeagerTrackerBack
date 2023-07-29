use serde::{Deserialize, Serialize};

// Structure de la carte répertoriant le les données de ma partie
#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    Controleur,
    Duelliste,
    Sentinelle,
    Initiateur
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameCard {
    id: Option<i32>,
    is_victory: bool,
    is_competitive: bool, // Un Enum serait aurait été plus juste ici
    character_name: String,
    kda: [i32; 3],
    role: Role,
    comment: String,
}