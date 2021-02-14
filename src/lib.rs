use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_create_string_utf8, napi_env,
    napi_set_named_property, napi_value,
};

use std::ffi::CString;
use std::{error::Error, fs, process};

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
    let files_from_dataset_dir = read_dataset_dir("./dataset");
    let mut files = vec![];

    for file in files_from_dataset_dir {
        let file_path = format!("dataset/{}", file.as_str());
        files.push(
        csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(file_path).expect("error: problem to create vector of files"));
    }

    let mut wtr = csv::Writer::from_path("src/final.csv")?;
    for mut file in files {
        for data in file.deserialize() {
            let f: Record = data?; 
            wtr.serialize( WFile{id: f.respondent, country: f.country.as_str()}).expect("error: problem while serializing");
        } 
        wtr.flush().expect("error: was not able to write file");
    }
    Ok(())
}

pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {
// creating  a javastring string
    match parse() {
        Ok(())=> {}
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
    let mut local: napi_value = std::mem::zeroed();
    let p = CString::new("\nCalling rust parsing function from node!!!\n").expect("CString::new    failed");
    if let Ok(i) =  p.to_str() {
        let p_size = i.len();
        // println!("{}", p_size);
        napi_create_string_utf8(env, p.as_ptr(), p_size, &mut local);
    }else {
        let default_msg_size = 50;
        napi_create_string_utf8(env, p.as_ptr(), default_msg_size, &mut local);
    }
// returning the javascript string
    local
}
#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
// creating a C String
    let p = CString::new("myFunc").expect("CString::new failed");
// creating a location where pointer to napi_value be written
    let mut local: napi_value = std::mem::zeroed();
    napi_create_function(
        env,
// pointer to function name
        p.as_ptr(),
// length of function name
        5,
// rust function
        Some(say_hello),
// context which can be accessed by the rust function
        std::ptr::null_mut(),
// output napi_value
        &mut local,
    );
// set function as property 
    napi_set_named_property(env, exports, p.as_ptr(), local);
// returning exports
    exports
}