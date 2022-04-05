pub trait Required {
    fn load_mod() -> Mod;
}

pub struct Mod {
    pub name: String,
    pub description: String,
    pub print: Option<extern "C" fn()>,
}
