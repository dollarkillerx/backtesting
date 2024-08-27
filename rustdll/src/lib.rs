use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use ureq;

#[no_mangle]
pub extern "system" fn  hello(count: i32) -> i32 {
    if count < 0 {
        return 0;
    }
    return 1;
}

#[no_mangle]
pub extern "system" fn hello2(cstr: *const c_char) {
    // Check if the pointer is null
    if cstr.is_null() {
        println!("hello2: Received a null pointer");
        return;
    }

    // Convert the C string pointer to a Rust &CStr
    let c_str = unsafe { CStr::from_ptr(cstr) };

    // Convert the &CStr to a Rust String
    match c_str.to_str() {
        Ok(string) => {
            // http://127.0.0.1:8181/hello
            let body: String = ureq::get(format!("http://127.0.0.1:8181/{}", string).as_str())
                .call().unwrap()
                .into_string().unwrap();

            println!("{}", body);
        },
        Err(e) => println!("hello2: Invalid UTF-8 sequence: {}", e),
    }
}