use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::DistrictItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommuneDBItem{
    pub id: Uuid,
    pub nom: String,
    pub district_id: Uuid
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct CommunesDBItems(pub Vec<CommuneDBItem>);

impl From<DistrictItem> for CommunesDBItems {
    fn from(value: DistrictItem) -> Self {
        Self(value.communes.into_inner().iter().map(|i| {
            CommuneDBItem { id: i.id, nom: i.nom.to_owned(), district_id : value.id}
        }).collect())
    }
}