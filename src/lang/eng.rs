use super::Lang;

pub struct LangEng;

impl LangEng {
    pub fn new() -> Lang {
        Lang {
            nav_home: "Home".to_string(),
            nav_search: "Search".to_string(),
            nav_collection: "Your Library".to_string(),
            create_playlist: "Create Playlist".to_string(),
            liked_songs: "Liked Songs".to_string(),
        }
    }
}