#[macro_use]
extern crate serde_derive;

use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_create_string_utf8, napi_env,
    napi_set_named_property, napi_value,
};

use std::process;
use std::ffi::CString;

mod parse;
use parse::parse;

pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {
    match parse(&["dataset", "src/final.csv"]) {
        Ok(()) => {}
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
    let mut local: napi_value = std::mem::zeroed();
    // creating  a javastring string
    let p = CString::new("\nCalling rust parsing function from node!!!\n")
        .expect("CString::new failed");
    if let Ok(i) = p.to_str() {
        let p_size = i.len();
        // println!("{}", p_size);
        napi_create_string_utf8(env, p.as_ptr(), p_size, &mut local);
    } else {
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
