use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidat {
    pub numero: u16,
    pub nom: String,
    pub partie: String,
    pub nombre: u32,
    pub pourcentage: String,
}
