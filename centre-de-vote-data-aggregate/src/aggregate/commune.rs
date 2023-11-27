use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{CentreVoteDataItem, Fokotanys};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CommuneItem{
    pub nom: String,
    pub id: Uuid,
    pub fokotanys: Fokotanys
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Communes(Vec<CommuneItem>);

impl Communes {
    pub fn into_inner(&self) -> &Vec<CommuneItem> {
        &self.0
    }
    pub fn into_inner_mut(&mut self) -> &mut Vec<CommuneItem> {
        &mut self.0
    }
    pub fn get_value(&self, input: &CentreVoteDataItem) -> Option<&Fokotanys> {
        let pos = self.into_inner().iter().position(|i| i.nom == input.commune)?;
        Some(&self.into_inner()
            .get(pos)?.fokotanys)
    }
    pub fn get_value_mut(&mut self, input: &CentreVoteDataItem) -> Option<&mut Fokotanys> {
        let key = self
            .into_inner()
            .iter()
            .position(|i| i.nom == input.commune)?;
        self.into_inner_mut().get_mut(key).map(|d| &mut d.fokotanys)
    }
    pub fn insert(&mut self, input: &CentreVoteDataItem) {
        if let Some(d) = self.get_value_mut(input) {
            d.insert(input);
        }else{
            let mut fokotanys: Fokotanys = Default::default();
            fokotanys.insert(input);
            self.0.push(CommuneItem {
                nom: input.commune.to_owned(),
                id: Uuid::new_v4(),
                fokotanys
            });
        }
    } 
}
