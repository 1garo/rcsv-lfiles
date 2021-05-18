#![feature(proc_macro_hygiene, decl_macro)]
use std::{error::Error, fs, process};

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;


#[macro_use] 
extern crate rocket;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

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
    let files_from_dataset_dir = read_dataset_dir("./dataset");
    let mut files = vec![];

    for file in files_from_dataset_dir {
        let file_path = format!("dataset/{}", file.as_str());
        files.push(csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(file_path).expect("error: problem to create vector of files"));
    }

    let mut wtr = csv::Writer::from_writer(vec![]);
    for mut file in files {
        for data in file.deserialize() {
            let f: Record = data?; 
            wtr.serialize( WFile{id: f.respondent, country: f.country.as_str()}).expect("error: problem while serializing");
        } 
        wtr.flush().expect("error: was not able to write file");
    }
    let data = String::from_utf8(wtr.into_inner()?)?;
    fs::write("./src/final.csv", data)?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    match parse() {
        Ok(())=> {
            "Hello, world!"
        }
        Err(_err) => {
            // format!("{}", err).0
            // println!("{}", err);
            process::exit(1);
        }
    }
}

fn main() {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::default())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(false);
    rocket::ignite().attach(cors.to_cors().unwrap());
    rocket::ignite().mount("/", routes![index]).launch();
}
