use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{CentreVoteDataItem, Districts};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RegionItem{
    pub nom: String,
    pub id: Uuid,
    pub districts: Districts
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Regions(Vec<RegionItem>);

impl Regions {
    pub fn into_inner(&self) -> &Vec<RegionItem> {
        &self.0
    }
    pub fn into_inner_mut(&mut self) -> &mut Vec<RegionItem> {
        &mut self.0
    }
    pub fn get_value(&self, input: &CentreVoteDataItem) -> Option<&Districts> {
        let pos = self.into_inner().iter().position(|i| i.nom == input.region)?;
        Some(&self.into_inner()
            .get(pos)?.districts)
    }
    pub fn get_value_mut(&mut self, input: &CentreVoteDataItem) -> Option<&mut Districts> {
        let key = self
            .into_inner()
            .iter()
            .position(|i| i.nom == input.region)?;
        self.into_inner_mut().get_mut(key).map(|d| &mut d.districts)
    }
    pub fn insert(&mut self, input: &CentreVoteDataItem) {
        if let Some(d) = self.get_value_mut(input) {
            d.insert(input);
        }else{
            let mut communes: Districts = Default::default();
            communes.insert(input);
            self.0.push(RegionItem {
                nom: input.region.to_owned(),
                id: Uuid::new_v4(),
                districts: communes
            });
        }
    } 
}

impl From<Vec<CentreVoteDataItem>> for Regions {
    fn from(value: Vec<CentreVoteDataItem>) -> Self {
        let mut regions = Self::default();
        value.iter().for_each(|input| regions.insert(input));
        regions
    }
}