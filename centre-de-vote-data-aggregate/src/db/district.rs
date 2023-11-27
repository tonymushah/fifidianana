use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::RegionItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DistrictDBItem{
    pub id: Uuid,
    pub nom: String,
    pub region_id: Uuid
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct DistrictDBItems(pub Vec<DistrictDBItem>);

impl From<RegionItem> for DistrictDBItems {
    fn from(value: RegionItem) -> Self {
        Self(value.districts.into_inner().iter().map(|i| {
            DistrictDBItem { id: i.id, nom: i.nom.to_owned(), region_id : value.id}
        }).collect())
    }
}