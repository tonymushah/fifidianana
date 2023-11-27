mod bureau_de_vote;
mod centre_vote;
mod fokotany;
mod commune;
mod district;
mod region;

pub use bureau_de_vote::*;
pub use centre_vote::*;
pub use fokotany::*;
pub use commune::*;
pub use district::*;
pub use region::*;

use serde::{Deserialize, Serialize};

use crate::{Regions, RegionItem};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BigDB{
    pub bureau_votes: Vec<BureauVoteBDItem>,
    pub centre_votes: Vec<CentreVoteDBItem>,
    pub fokotanys: Vec<FokotanyDBItem>,
    pub communes: Vec<CommuneDBItem>,
    pub district: Vec<DistrictDBItem>,
    pub region: Vec<RegionDBItem>
}

impl From<Regions> for BigDB{
    fn from(value: Regions) -> Self {
        let mut big_db = Self::default();
        value.into_inner().iter().for_each(|region| {
            region.districts.into_inner().iter().for_each(|district| {
                district.communes.into_inner().iter().for_each(|commune| {
                    commune.fokotanys.into_inner().iter().for_each(|fokotany| {
                        fokotany.centre_votes.into_inner().iter().for_each(|centre_vote|{
                            big_db.bureau_votes.append(&mut std::convert::Into::<BureauVoteBDItems>::into(centre_vote.clone()).0)
                        });
                        big_db.centre_votes.append(&mut std::convert::Into::<CentreVoteBDItems>::into(fokotany.clone()).0);
                    });
                    big_db.fokotanys.append(&mut std::convert::Into::<FokotanyDBItems>::into(commune.clone()).0);
                });
                big_db.communes.append(&mut std::convert::Into::<CommunesDBItems>::into(district.clone()).0)
            });
            big_db.district.append(&mut std::convert::Into::<DistrictDBItems>::into(region.clone()).0);
            big_db.region.push(<RegionDBItem as From<RegionItem>>::from(region.clone()));
        });
        big_db
    }
}