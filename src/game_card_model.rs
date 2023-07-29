use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Outcome {
    Victoire,
    Défaite,
    Egalité,
    Annulé
}

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Character {
    Astra,
    Breach,
    Brimstone,
    Chamber,
    Cypher,
    Deadlock,
    Fade,
    Gekko,
    Harbor,
    Jett,
    KayO,
    Killjoy,
    Neon,
    Omen,
    Pheonix,
    Raze,
    Reyna,
    Sage,
    Syke,
    Sova,
    Viper,
    Yoru
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    Controleur,
    Duelliste,
    Sentinelle,
    Initiateur
}

// Fonction getter/setter
impl GameCard {
    //pub fn get_id(&mut self) { return self.id; }
    pub fn get_id(&self) -> Option<i32> { return self.id; }
    pub fn get_outcome(&self) -> Outcome { return self.outcome; }
    pub fn get_game_type(&self) -> GameType { return self.game_type; }
    pub fn get_character(&self) -> Character { return self.character; }
    pub fn get_kda(&self) -> [i32; 3] { return self.kda; }
    pub fn get_role(&self) -> Role { return self.role; }
    pub fn get_comment(&self) -> &String { return &self.comment; }

    pub fn set_id(&mut self, new_id: Option<i32>) { self.id = new_id; }
    pub fn set_outcome(&mut self, outcome: Outcome) { self.outcome = outcome; }
    pub fn set_game_type(&mut self, game_type: GameType) { self.game_type = game_type; }
    pub fn set_character(&mut self, character: Character) { self.character = character; }
    pub fn set_kda(&mut self, kda: [i32; 3]) { self.kda = kda; }
    pub fn set_role(&mut self, role: Role) { self.role = role; }
    pub fn set_comment(&mut self, comment: &String) { self.comment = comment.clone(); }
}

// Structure de la carte répertoriant le les données de ma partie
#[derive(Clone, Serialize, Deserialize)]
pub struct GameCard {
    id: Option<i32>,
    outcome: Outcome,
    game_type: GameType,
    character: Character,
    kda: [i32; 3],
    role: Role,
    comment: String,
}