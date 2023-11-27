use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::CentreVoteItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BureauVoteBDItem{
    pub code: usize,
    pub nom: String,
    pub centre_vote_id: Uuid
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct BureauVoteBDItems(pub Vec<BureauVoteBDItem>);

impl From<CentreVoteItem> for BureauVoteBDItems {
    fn from(value: CentreVoteItem) -> Self {
        Self(value.bureau_votes.iter().map(|i| {
            BureauVoteBDItem { code: i.code, nom: i.nom.to_owned(), centre_vote_id: value.id }
        }).collect())
    }
}