extern crate csv;
extern crate serde;
use std::{error::Error, fs};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    respondent: u64,
    country: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WFile<'a> {
    id: u64,
    country: &'a str,
}

fn read_dataset_dir(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).expect("error: problem to read dataset path");
    let names = paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();
    names
}

pub fn parse(files: &[&str; 2]) -> Result<(), Box<dyn Error>> {
    let [files_dir, csv_src_file] = files;
    let files_from_dataset_dir = read_dataset_dir(files_dir);
    let mut files = vec![];

    for file in files_from_dataset_dir {
        let file_path = format!("{}/{}", files_dir, file.as_str());
        files.push(
            csv::ReaderBuilder::new()
                .flexible(true)
                .from_path(file_path)
                .expect("error: problem to create vector of files"),
        );
    }

    let mut wtr = csv::Writer::from_path(csv_src_file).expect("cannot find or read final.csv file");
    for mut file in files {
        for data in file.deserialize() {
            let f: Record = data.expect("cannot serialize data from file");
            wtr.serialize(WFile {
                id: f.respondent,
                country: f.country.as_str(),
            })
            .expect("error: problem while serializing");
        }
        wtr.flush().expect("error: was not able to write file");
    }
    Ok(())
}