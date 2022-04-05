use modloader_api::*;
use std::fs;

fn main() {
    let mut loaded_libs = Vec::new();
    let mut loaded_mods = Vec::new();

    for file in fs::read_dir("./mods/").unwrap() {
        unsafe {
            loaded_libs.push(libloading::Library::new(file.unwrap().path()).unwrap());
            let load_mod: libloading::Symbol<extern "C" fn() -> Mod> =
                loaded_libs.last().unwrap().get(b"load_mod").unwrap();
            loaded_mods.push(load_mod());
        }
    }

    for loaded_mod in loaded_mods {
        println!(
            "Loaded mod: {} ({})",
            loaded_mod.name, loaded_mod.description
        );
        if let Some(mod_print_fn) = loaded_mod.print {
            mod_print_fn();
        } else {
            println!("Does not contain print funciton!");
        }
        println!("");
    }
}
