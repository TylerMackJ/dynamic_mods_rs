use modloader_api::*;
use std::ffi::CString;
use example_api::ExampleAPI;

load_mod!{Mod {
    name: CString::new("Example Mod").unwrap(),
    description: CString::new("This is an example mod").unwrap(),
    print: Some(example_print),
}}

extern "C" fn example_print() {
    let get_example_api = get_api!(CString::new("Example API").unwrap()) as *const fn() -> ExampleAPI;
    unsafe {
        let api = (*get_example_api)();
        (api.api_call)();
    }
    
    println!("This is from the example mod");
}