use modloader_api::*;

fn main() {
    unsafe {
        let lib = libloading::Library::new("./mods/libhello_world_mod.so").unwrap();
        let load_mod: libloading::Symbol<unsafe extern "C" fn() -> Mod> =
            lib.get(b"load_mod").unwrap();
        let loaded_mod = load_mod();

        println!(
            "Name: {}\nDescription: {}",
            loaded_mod.name, loaded_mod.description
        );

        if let Some(print_fn) = loaded_mod.print {
            print_fn();
        }
    }
}
