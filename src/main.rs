use std::{error::Error, io, process};
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    respondent: u64,
    country: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WFile<'a>{
    id: u64,
    country: &'a str,
}

fn parse(path_r: &str) -> Result<(), Box<dyn Error>> {
    //TODO: read database folder for .csv files and not receive it as a param
    let rdr= csv::ReaderBuilder::new()
    .flexible(true)
    .from_path(path_r)?;
    let rdr2= csv::ReaderBuilder::new()
    .flexible(true)
    .from_path("dataset/survey_results_public.csv")?;
    let files = vec![rdr, rdr2];
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for mut file in files {
        for data in file.deserialize() {
            let f: Record = data?; 
                wtr.serialize( WFile{id: f.respondent, country: f.country.as_str()})?;
        } 
        wtr.flush()?;
    }
    Ok(())
}

fn main() {
    //TODO: finish and make the README greater
    match parse("dataset/2017.csv") {
        Ok(())=> {}
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
