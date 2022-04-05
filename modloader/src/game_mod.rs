pub trait Required {
    //const NAME: &'static str;
    //const DESCRIPTION: &'static str;

    fn load_mod() -> Mod;
}

pub struct Mod {
    pub name: String,
    pub description: String,
    pub print: Option<unsafe extern "C" fn()>,
}
