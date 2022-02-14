pub mod router;

#[derive(Clone, PartialEq, Debug)]
pub enum WidthColumn {
    EqualPx,
    Relative,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ContentPlaylist {
    pub url: String,
    pub name: String
}