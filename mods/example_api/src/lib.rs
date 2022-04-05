use modloader_api::*;
use std::ffi::CString;

load_mod!{Mod {
    name: CString::new("Example API").unwrap(),
    description: CString::new("This is an example API").unwrap(),
    print: None,
}}

pub struct ExampleAPI {
    pub api_call: extern "C" fn(),
}

#[no_mangle]
fn get_api() -> ExampleAPI {
    ExampleAPI {
        api_call: api_call,
    }
}

extern "C" fn api_call() {
    println!("This is an api_call");
}