use std::ffi::CString;

#[macro_export]
macro_rules! load_mod {
    ( $x:expr ) => {
        static mut MOD_TOOLS: Option<ModTools> = None;

        #[no_mangle]
        fn load_mod(mod_tools: ModTools) -> Mod {
            unsafe{ MOD_TOOLS = Some(mod_tools) };
            $x
        }
    }
}

#[macro_export]
macro_rules! get_api {
    ( $x:expr ) => {
        unsafe {
            (MOD_TOOLS.unwrap().get_api)($x)
        }
    }
}


pub struct Mod {
    pub name: CString,
    pub description: CString,
    pub print: Option<extern "C" fn()>,
}

#[derive(Copy, Clone)]
pub struct ModTools {
    pub get_api: extern "C" fn(CString) -> libloading::Symbol,
}