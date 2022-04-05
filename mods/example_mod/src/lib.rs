use modloader_api::*;

struct HelloWorldMod {}

impl Required for HelloWorldMod {
    #[no_mangle]
    fn load_mod() -> Mod {
        Mod {
            name: "Example Mod".to_string(),
            description: "This is an example mod".to_string(),
            print: None,
        }
    }
}
