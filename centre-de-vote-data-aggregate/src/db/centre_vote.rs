use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::FokotanyItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CentreVoteDBItem{
    pub id: Uuid,
    pub nom: String,
    pub fokotany_id: Uuid
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct CentreVoteBDItems(pub Vec<CentreVoteDBItem>);

impl From<FokotanyItem> for CentreVoteBDItems {
    fn from(value: FokotanyItem) -> Self {
        Self(value.centre_votes.into_inner().iter().map(|i| {
            CentreVoteDBItem { id: i.id, nom: i.nom.to_owned(), fokotany_id : value.id}
        }).collect())
    }
}