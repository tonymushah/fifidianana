use serde::{Deserialize, Serialize};

use crate::CentreVoteDataItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BureauVote{
    pub code: usize,
    pub nom: String
}

impl From<CentreVoteDataItem> for BureauVote {
    fn from(value: CentreVoteDataItem) -> Self {
        Self { code: value.code_bv, nom: value.bureau_vote }
    }
}
