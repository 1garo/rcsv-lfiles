use std::{error::Error, fs, io, process};

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

fn read_dataset_dir(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).expect("error: problem to read dataset path");
    let names =
    paths.filter_map(|entry| {
      entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)))
      )
    }).collect::<Vec<String>>();
    names
}

fn parse() -> Result<(), Box<dyn Error>> {
    let files_from_dir = read_dataset_dir("./dataset");
    let mut files = vec![];

    for file in files_from_dir {
        let file_name = format!("dataset/{}", file.as_str());
        files.push(csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(file_name).expect("error: problem to create vector of files"));
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());
    for mut file in files {
        for data in file.deserialize() {
            let f: Record = data?; 
            wtr.serialize( WFile{id: f.respondent, country: f.country.as_str()}).expect("error: problem while serializing");
        } 
        wtr.flush().expect("error: was not able to write file");
    }
    Ok(())
}

fn main() {
    match parse() {
        Ok(())=> {}
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
