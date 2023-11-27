use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::CommuneItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FokotanyDBItem{
    pub id: Uuid,
    pub nom: String,
    pub fokotany_id: Uuid
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct FokotanyDBItems(pub Vec<FokotanyDBItem>);

impl From<CommuneItem> for FokotanyDBItems {
    fn from(value: CommuneItem) -> Self {
        Self(value.fokotanys.into_inner().iter().map(|i| {
            FokotanyDBItem { id: i.id, nom: i.nom.to_owned(), fokotany_id : value.id}
        }).collect())
    }
}