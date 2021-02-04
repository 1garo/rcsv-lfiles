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
struct WFile {
    id: u64,
    country: String,
}

// fn w_file(path: &str, data: csv::Writer<Vec<u8>>) -> Result<(), Box<dyn Error>> {
//     let mut wtr = csv::WriterBuilder::new()
//         .from_path(path)?;
//     let mut record = csv::StringRecord::new();
//     wtr.write_record(&[data])?;
//     wtr.flush()?;
//     Ok(())
// }

// fn read_and_write(
//     in_file_path: &str,
//     out_file_path: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_path(in_file_path)?;
//     let mut wtr = csv::WriterBuilder::new()
//         .from_path(out_file_path)?;

// let mut record = csv::ByteRecord::new();
// while rdr.read_byte_record(&mut record)? {
//     wtr.write_byte_record(&record)?;
// }
//     wtr.flush()?;

//     Ok(())
// }

// fn concat() -> Result<(), Box<dyn Error>> {
//     let foo = File::open("dataset/2017.csv")?;
//     let bar = File::open("dataset/2018.csv")?;
//     let files = [foo, bar];
//     let mut c = ConcatReader::new(&files);
//     let mut data: Vec<u8> = vec![];
//     c.read_to_end(&mut data)?; 
//     use std::fs::OpenOptions;
//     let file = OpenOptions::new()
//     .write(true)
//     .create(true)
//     .append(true)
//     .open("dataset/final_concat.csv")
//     .unwrap();
//     let mut wtr = csv::Writer::from_writer(file);
//     wtr.write_byte_record(&csv::ByteRecord::from(vec![data]))?;
//     wtr.flush()?;
//     // fs::write("dataset/final_concat.csv", data)?;
//     Ok(())
// }

fn merda(path_r: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = OpenOptions::new()
    .write(true)
    .append(true)
    .open("dataset/test.csv")
    .unwrap();
    let mut file = OpenOptions::new()
    .append(true)
    .open("dataset/2017.csv")
    .unwrap();
    let mut file2= OpenOptions::new()
    .write(true)
    .append(true)
    .open("dataset/2018.csv")
    .unwrap();
    let mut rdr= csv::Reader::from_reader(file);
    let mut rdr2= csv::Reader::from_reader(file2);
    let files = vec![rdr, rdr2];
    for file in files {

    }
    // wtr.flush()?;
    Ok(())
}
fn run(path_r: &str) -> Result<u64, Box<dyn Error>> {
    // let mut rdr = csv::ReaderBuilder::new()
    //     .has_headers(false)
    //     .from_path(path_r)?;
    let mut rdr= csv::ReaderBuilder::new()
    .flexible(true)
    .from_path(path_r)?;
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let count = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("teste: {:?}", record);
        wtr.serialize( WFile{id: record.respondent, country: record.country})?;

    //     wtr.write_record( WFile{id: record.respondent, country: record.country})?;
    //     // let data = String::from_utf8(wtr.into_inner()?)?;
    //     // wrt.write_record(data);
    //     // wtr.write_record( &csv::StringRecord::from(vec![WFile{id: record.respondent, country: record.country}]))?;
    //     // wtr.write_byte_record(&record)?;
    } 
    wtr.flush()?;
    // let mut record = csv::StringRecord::new();
    // while rdr.read_record(&mut record)? {
    //     println!("{}", record);
    //     // record.get(i)
    //     // wtr.write_record( WFile{id: record.respondent, country: record.country});
    //     // wtr.write_record(&record.index("repondent"))?;
    // }
    // w_file("dataset/final.csv", wtr);
    // fs::write("/tmp/final.csv", wtr).expect("Unable to write file");
    Ok(count)
}

fn main() {
    // read_and_write("dataset/2017.csv", "dataset/final.csv").unwrap();
    // concat();
    //TODO: merge the files and exec run function on merged file
   merda(); 
    match run("dataset/final_concat.csv") {
        Ok(count) => {
            println!("{}", count);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
