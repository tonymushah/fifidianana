use std::{
    ffi::OsString,
    fs::{read_dir, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::exit,
};

fn preprocess(output: PathBuf, input: PathBuf) -> io::Result<()> {
    let files: Vec<(File, OsString)> = read_dir(input)?
        .flat_map(|e| {
            let f: io::Result<(File, OsString)> = {
                let re = e?;
                let file = File::open(re.path())?;
                let file_name = re.file_name();
                Ok((file, file_name))
            };
            f
        })
        .collect();
    files.iter().for_each(|(file, filename)| {
        let res = || -> io::Result<()> {
            let mut output_ = output.clone();
            output_.push(filename);
            let reader = BufReader::new(file).lines();
            let mut writer = BufWriter::new(File::create(output_)?);
            reader.flatten().enumerate().for_each(|(line, text)| {
                if line > 2 && line < 57 {
                    let text = text.replace("150 elus", "");
                    //let text = text.replace("FTT", "FTT\n");
                    let text = text.replace("MMM", "MMM\n");
                    let text = text.replace("TGV", "TGV\n");
                    let text = text.replace("MTS", "MTS\n");
                    let text = text.replace("TIM", "TIM\n");
                    let text = text.replace("TT", "TT\n");
                    let text = text.replace("ARB", "ARB\n");
                    let text = text.replace("APM", "APM\n");
                    let text = text.replace(
                        "Antoko Fihavanantsika An'i Kristy",
                        "Antoko Fihavanantsika An'i Kristy\n",
                    );
                    let text = text.replace("HVM", "HVM\n");
                    let text = text.replace("Fy-M", "Fy-M\n");
                    let text = text.replace("FMI-Ma", "FMI-Ma\n");
                    let text = text.replace(
                        "Fitambolagnela/IAD , PSD , RPSD Vaovao , ABA, &parrainage de ",
                        "Fitambolagnela/IAD, PSD, RPSD Vaovao, ABA",
                    );
                    let text = text.replace(
                        "RAJAONARIMAMPIANINA RAKOTOARIMANANA Hery-",
                        "RAJAONARIMAMPIANINA RAKOTOARIMANANA Hery Martial",
                    );
                    let text = text.replace("Martial", "");
                    if !text.is_empty() {
                        if let Some(e) = writeln!(&mut writer, "{text}").err() {
                            eprintln!("{}", e);
                        }
                    }
                }
            });
            Ok(())
        };
        if let Err(e) = res() {
            eprintln!("{}", e);
        }
    });
    Ok(())
}

fn to_csv(input: PathBuf, output: PathBuf) -> io::Result<()> {
    let files: Vec<(File, String)> = read_dir(input)?
        .flat_map(|e| {
            let f: io::Result<(File, String)> = {
                let re = e?;
                let file = File::open(re.path())?;
                let file_name = re
                    .file_name()
                    .to_str()
                    .ok_or(io::Error::new(
                        io::ErrorKind::InvalidData,
                        String::from("Can't convert Ostring to string"),
                    ))?
                    .to_string();
                Ok((file, file_name))
            };
            f
        })
        .collect();
    files.iter().for_each(|(file, filename)| {
        let filename = filename.replace(".pdf.txt", ".csv");
        let res = || -> io::Result<()> {
            let mut output_ = output.clone();
            output_.push(filename);
            let reader = BufReader::new(file).lines();
            let mut writer = BufWriter::new(File::create(output_)?);
            //let mut seq = 1;
            if let Ok(()) = writeln!(&mut writer, "numero;nom;partie;nombre;pourcentage") {
                reader.flatten().enumerate().for_each(|(line, text)| {
                if (line + 1) % 4 == 0 {
                    let text: String = text.replace(' ', ";");
                    if let Some(e) = writeln!(&mut writer, "{text}").err() {
                        eprintln!("{}", e);
                    }
                } else if let Some(e) = write!(&mut writer, "{text};").err() {
                    eprintln!("{}", e);
                }
            });
            }
            
            Ok(())
        };
        if let Err(e) = res() {
            eprintln!("{}", e);
        }
    });
    Ok(())
}

fn run_preprocess() -> io::Result<()> {
    let input = Path::new("./data/bv2a.zip-20231127T072731Z-001/bv2a");
    let output = Path::new("./data/bv2a").to_path_buf();

    preprocess(output.to_path_buf(), input.to_path_buf())?;
    Ok(())
}

fn run_to_csv() -> io::Result<()> {
    let input = Path::new("./data/bv2a").to_path_buf();
    let output = Path::new("./data/bv2a_csv").to_path_buf();
    to_csv(input, output)?;
    Ok(())
}

fn run() -> io::Result<()> {
    run_preprocess()?;
    run_to_csv()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running example: {}", err);
        exit(1);
    }
}
