use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::RegionItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegionDBItem{
    pub id: Uuid,
    pub nom: String,
}

impl From<RegionItem> for RegionDBItem {
    fn from(value: RegionItem) -> Self {
        Self { id: value.id, nom: value.nom }
    }
}