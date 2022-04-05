use modloader_api::*;
use std::fs;
use std::ffi::CString;

struct LoadedMod {
    library: libloading::Library,
    mod_data: Mod,
}

static mut LOADED_MODS: Vec<LoadedMod> = Vec::new();

fn main() {
    for file in fs::read_dir("./mods/").unwrap() {
        unsafe {
            // Get Lib
            let library = libloading::Library::new(file.unwrap().path()).unwrap();

            // Get function pointer to load_mod
            let load_mod: libloading::Symbol<extern "C" fn(ModTools) -> Mod> = library.get(b"load_mod\0").unwrap();
            
            let mod_tools: ModTools = ModTools {
                get_api: get_api,
            };

            // Load the mod
            let mod_data = load_mod(mod_tools);

            LOADED_MODS.push(LoadedMod {
                library: library, 
                mod_data: mod_data,
            })
        }
    }

    unsafe {
        for loaded_mod in &LOADED_MODS {
            println!(
                "Loaded mod: {:?} ({:?})",
                loaded_mod.mod_data.name, loaded_mod.mod_data.description
            );
            if let Some(mod_print_fn) = loaded_mod.mod_data.print {
                mod_print_fn();
            } else {
                println!("Does not contain print funciton!");
            }
            println!("");
        }
    }
}

extern "C" fn get_api(mod_name: CString) -> libloading::Symbol {
    unsafe {
        for loaded_mod in &LOADED_MODS {
            if loaded_mod.mod_data.name == mod_name {
                return loaded_mod.library.get::<fn() -> Mod>(b"get_api\0").unwrap();
            }
        }
    }
    panic!("Api not found!");
}