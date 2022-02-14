pub mod container;
pub mod header;
pub mod track_list;
pub mod btn_play;
pub mod nav;
pub mod playlist;
pub mod player;
pub mod track;
pub mod layout;
pub mod layouts;
pub mod context;
pub mod logo;
pub mod grid;
pub mod user_list;
pub mod icons;
pub mod control_list;
pub mod control_playlist;

pub mod prelude {
    pub use super::container::Container;
    pub use super::header::Header;
    pub use super::track_list::TrackList;
    pub use super::btn_play::ButtonPlay;
    pub use super::nav::Nav;
    pub use super::playlist::Playlist;
    pub use super::player::Player;
    pub use super::track::Track;
    pub use super::layout::Layout;
    pub use super::layouts::user_content::UserContent;
    pub use super::layouts::user_page::UserPage;
    pub use super::context::{AppContext, AppContextProvider, AppContextProps};
    pub use super::logo::Logo;
    pub use super::user_list::UserList;
    pub use super::control_list::ControlList;
    pub use super::control_playlist::ControlPlaylist;
}