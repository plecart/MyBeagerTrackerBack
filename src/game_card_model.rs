use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    Controleur,
    Duelliste,
    Sentinelle,
    Initiateur
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameType {
    NonClassé,
    Compétition,
    Vélocité,
    SpikeRush,
    CombatAMort,
    Intensification,
    CombatAMortPartEquipe,
    PartiePersonnnalisé
}

// Fonction pour généré automatiquement un id a chaque partie
impl GameCard {
    pub fn set_id(&mut self, new_id: Option<i32>) {
        self.id = new_id;
    }
}

// Structure de la carte répertoriant le les données de ma partie
#[derive(Clone, Serialize, Deserialize)]
pub struct GameCard {
    id: Option<i32>,
    is_victory: bool,
    game_type: GameType, // Un Enum serait aurait été plus juste ici
    character_name: String,
    kda: [i32; 3],
    role: Role,
    comment: String,
}