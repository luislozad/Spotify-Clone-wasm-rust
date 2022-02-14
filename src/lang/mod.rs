pub mod eng;
pub mod es;

#[derive(Clone, PartialEq, Debug)]
pub struct Lang {
    pub nav_home: String,
    pub nav_search: String,
    pub nav_collection: String,
    pub create_playlist: String,
    pub liked_songs: String,
}

pub mod prelude {
    pub use super::Lang;
    pub use super::es::LangEs;
    pub use super::eng::LangEng;
}