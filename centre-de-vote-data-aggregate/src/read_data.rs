use std::{path::PathBuf, fs::File, io::BufReader};
use csv::ReaderBuilder;

use crate::{CentreVoteDataItem, Result};

pub fn read_data(path: PathBuf) -> Result<Vec<CentreVoteDataItem>> {
    let file_buf = BufReader::new(File::open(path)?);
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file_buf);
    Ok(reader.deserialize::<CentreVoteDataItem>().flatten().collect::<Vec<CentreVoteDataItem>>())
}