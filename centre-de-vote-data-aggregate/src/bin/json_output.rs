use std::{path::Path, process::exit, io::BufWriter, fs::File};

use centre_de_vote_data_aggregate::{read_data, Result, Regions, db::BigDB};

fn run() -> Result<()> {
    let path = Path::new("./data/centre-de-vote-data.csv");
    let aggregate: Regions = read_data(path.to_path_buf())?.into();
    let output_buf = BufWriter::new(File::create(Path::new("./output.json"))?);
    serde_json::to_writer_pretty(output_buf, &<BigDB as From<Regions>>::from(aggregate))?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running example: {}", err);
        exit(1);
    }
}
