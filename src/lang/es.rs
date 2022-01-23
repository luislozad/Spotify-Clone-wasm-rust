use super::Lang;

pub struct LangEs;

impl LangEs {
    pub fn new() -> Lang {
        Lang {
            nav_home: "Inicio".to_string(),
            nav_search: "Buscar".to_string(),
            nav_collection: "Tu Biblioteca".to_string(),
        }
    }
}