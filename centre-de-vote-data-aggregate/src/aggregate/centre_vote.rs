use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{BureauVote, CentreVoteDataItem};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CentreVoteItem{
    pub nom: String,
    pub id: Uuid,
    pub bureau_votes: Vec<BureauVote>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct CentreVotes(Vec<CentreVoteItem>);

impl CentreVotes {
    pub fn into_inner(&self) -> &Vec<CentreVoteItem> {
        &self.0
    }
    pub fn into_inner_mut(&mut self) -> &mut Vec<CentreVoteItem> {
        &mut self.0
    }
    pub fn get_value(&self, input: &CentreVoteDataItem) -> Option<&Vec<BureauVote>> {
        let pos = self.into_inner().iter().position(|i| i.nom == input.centre_vote)?;
        Some(&self.into_inner()
            .get(pos)?.bureau_votes)
    }
    pub fn get_value_mut(&mut self, input: &CentreVoteDataItem) -> Option<&mut Vec<BureauVote>> {
        let key = self
            .into_inner()
            .iter()
            .position(|i| i.nom == input.centre_vote)?;
        self.into_inner_mut().get_mut(key).map(|d| &mut d.bureau_votes)
    }
    pub fn insert(&mut self, input: &CentreVoteDataItem) {
        if let Some(d) = self.get_value_mut(input) {
            d.push(input.to_owned().into());
        }else{
            let bureau_votes: Vec<BureauVote> = vec![input.clone().into()];
            self.0.push(CentreVoteItem {
                nom: input.centre_vote.to_owned(),
                id: Uuid::new_v4(),
                bureau_votes
            });
        }
    } 
}
