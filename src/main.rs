use std::{error::Error, io, process};
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


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.country == "us" && record.region == "MA" {
            count += 1;
        }
    }
    Ok(count)
}

fn main() {
    match run() {
        Ok(count) => {
            println!("{}", count);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
