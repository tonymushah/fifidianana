use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CentreVoteDataItem{
    pub region: String,
    pub district: String,
    pub commune: String,
    pub fokotany: String,
    pub centre_vote: String,
    pub code_bv: usize,
    pub bureau_vote: String
}
