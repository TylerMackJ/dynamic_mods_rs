use modloader_api::*;

struct HelloWorldMod {}

impl Required for HelloWorldMod {
    #[no_mangle]
    fn load_mod() -> Mod {
        Mod {
            name: "Hello World Mod".to_string(),
            description: "This is the Hello World Mod".to_string(),
            print: Some(random_mod_print),
        }
    }
}

extern "C" fn random_mod_print() {
    println!("Hello from the mod!");
}
