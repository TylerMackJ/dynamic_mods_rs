mod game_mod;

struct HelloWorldMod {}

impl game_mod::Required for HelloWorldMod {
    #[no_mangle]
    fn load_mod() -> game_mod::Mod {
        game_mod::Mod {
            name: "Hello World Mod".to_string(),
            description: "This is the Hello World Mod".to_string(),
            print: Some(random_mod_print),
        }
    }
}

extern "C" fn random_mod_print() {
    println!("Hello from the mod!");
}
