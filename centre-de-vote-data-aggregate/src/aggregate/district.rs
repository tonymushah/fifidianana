use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{CentreVoteDataItem, Communes};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DistrictItem{
    pub nom: String,
    pub id: Uuid,
    pub communes: Communes
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Districts(Vec<DistrictItem>);

impl Districts {
    pub fn into_inner(&self) -> &Vec<DistrictItem> {
        &self.0
    }
    pub fn into_inner_mut(&mut self) -> &mut Vec<DistrictItem> {
        &mut self.0
    }
    pub fn get_value(&self, input: &CentreVoteDataItem) -> Option<&Communes> {
        let pos = self.into_inner().iter().position(|i| i.nom == input.district)?;
        Some(&self.into_inner()
            .get(pos)?.communes)
    }
    pub fn get_value_mut(&mut self, input: &CentreVoteDataItem) -> Option<&mut Communes> {
        let key = self
            .into_inner()
            .iter()
            .position(|i| i.nom == input.district)?;
        self.into_inner_mut().get_mut(key).map(|d| &mut d.communes)
    }
    pub fn insert(&mut self, input: &CentreVoteDataItem) {
        if let Some(d) = self.get_value_mut(input) {
            d.insert(input);
        }else{
            let mut communes: Communes = Default::default();
            communes.insert(input);
            self.0.push(DistrictItem {
                nom: input.district.to_owned(),
                id: Uuid::new_v4(),
                communes
            });
        }
    } 
}
