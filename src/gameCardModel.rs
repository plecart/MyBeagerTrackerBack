// Structure de la carte répertoriant le les données de ma partie
enum Role {
    Controleur,
    Duelliste,
    Sentinelle,
    Initiateur
}

#[derive(Clone, Serialize, Deserialize)]
struct Card {
    id: Option<i32>,
    isVictory: Bool,
    isCompetitive: Bool, // Un Enum serait aurait été plus juste ici
    characterName: String,
    kda: [i32; 3],
    role: Role,
    comment: String,
}