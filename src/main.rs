use std::{borrow::Borrow, collections::HashMap, env, error::Error, fs::{self, OpenOptions}, io, ops::Index, process};

use concat_reader::*;
use io::Read;
use std::fs::File;

use csv::Writer;
// use serde::Deserialize;
// extern crate csv;
// struct Record {
//     city: 
// }

/* read using StringRecord approach
 fn read() -> Result<u64,Box<dyn Error>> {
     let mut rdr = csv::Reader::from_reader(io::stdin()); 
     let mut i = 0;
     for result in rdr.records() {
         let record = result?;
         if &record[0] == "us" && &record[3] == "MA" {
             i += 1;
         }
     }
     Ok(i)
 }
*/

/* read using ButeRecord approach
 fn read() -> Result<u64, Box<dyn Error>> {
     let mut rdr = csv::Reader::from_reader(io::stdin());

     let mut count = 0;
     for result in rdr.byte_records() {
         let record = result?;
         if &record[0] == b"us" && &record[3] == b"MA" {
             count += 1;
         }
     }
     Ok(count)
 }
*/

/* 
 - read using read byte record approach 
 - create a single ByteRecord and the csv read into it
*/
// fn read() -> Result<u64,Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     let mut record = csv::ByteRecord::new();

//     let mut count = 0;
//     while rdr.read_byte_record(&mut record)? {
//         if &record[0] == b"us" && &record[3] == b"MA" {
//             count += 1;
//         }
//     }
//     Ok(count)
// }

// fn main() {
//     match read() {
//         Ok(i) => {
//             println!("{}", i);
//         }
//         Err(err) => {
//             println!("{}", err);
//             process::exit(1);
//         }
//     }
// }

// TODO: reddit post used as support for learning how to read big files with rust
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
// #[serde(rename_all = "PascalCase")]
struct WFile<'a>{
    id: u64,
    country: &'a str,
}

fn run(path_r: &str) -> Result<(), Box<dyn Error>> {
    let rdr= csv::ReaderBuilder::new()
    .flexible(true)
    .from_path(path_r)?;
    let rdr2= csv::ReaderBuilder::new()
    .flexible(true)
    .from_path("dataset/survey_results_public.csv")?;
    let files = vec![rdr, rdr2];
    let mut wtr = csv::Writer::from_writer(io::stdout());
    // let mut headers_written = false;
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
    //TODO: find a way to continue the id counting instead of return to 1 when change files 
    //merda(); 
    match run("dataset/2017.csv") {
        Ok(()) => {}
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
