mod conversion;
use std::os::windows::ffi::OsStringExt;
use serde_json::json;
use ureq;
use crate::conversion::{double_ptr_to_vec, wide_ptr_to_string};

#[no_mangle]
pub extern "system" fn hello(count: i32) -> i32 {
    if count < 0 {
        return 0;
    }
    return 1;
}

#[no_mangle]
pub extern "system" fn fn_fill_array(arr: *mut f64, arr_size: i32) {
   let resp = double_ptr_to_vec(arr, arr_size).unwrap();
    let body: String = ureq::get(&format!("http://127.0.0.1:8181/{}", json!(resp).to_string()))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    println!("{}", body);
}

#[no_mangle]
pub extern "system" fn hello2(text: *mut u16) {
    if let Some(string) = wide_ptr_to_string(text) {
        println!("Text: {}", string);

        let body: String = ureq::get(&format!("http://127.0.0.1:8181/{}", string))
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        println!("{}", body);
    } else {
        println!("Received a null pointer");
    }
}

