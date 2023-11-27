use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{CentreVoteDataItem, CentreVotes};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FokotanyItem{
    pub nom: String,
    pub id: Uuid,
    pub centre_votes: CentreVotes
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Fokotanys(Vec<FokotanyItem>);

impl Fokotanys {
    pub fn into_inner(&self) -> &Vec<FokotanyItem> {
        &self.0
    }
    pub fn into_inner_mut(&mut self) -> &mut Vec<FokotanyItem> {
        &mut self.0
    }
    pub fn get_value(&self, input: &CentreVoteDataItem) -> Option<&CentreVotes> {
        let pos = self.into_inner().iter().position(|i| i.nom == input.fokotany)?;
        Some(&self.into_inner()
            .get(pos)?.centre_votes)
    }
    pub fn get_value_mut(&mut self, input: &CentreVoteDataItem) -> Option<&mut CentreVotes> {
        let key = self
            .into_inner()
            .iter()
            .position(|i| i.nom == input.fokotany)?;
        self.into_inner_mut().get_mut(key).map(|d| &mut d.centre_votes)
    }
    pub fn insert(&mut self, input: &CentreVoteDataItem) {
        if let Some(d) = self.get_value_mut(input) {
            d.insert(input);
        }else{
            let mut centre_votes: CentreVotes = CentreVotes::default();
            centre_votes.insert(input);
            self.0.push(FokotanyItem {
                nom: input.fokotany.to_owned(),
                id: Uuid::new_v4(),
                centre_votes
            });
        }
    } 
}
