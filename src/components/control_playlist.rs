use yew::prelude::*;

use crate::types::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PropsControlPlaylist {
    pub lists: Vec<ContentPlaylist>,
}

#[derive(Properties, Clone, PartialEq)]
struct PropsPlaylistItem {
    pub item: ContentPlaylist,
}

#[function_component(PlaylistItem)]
fn playlist_item(props: &PropsPlaylistItem) -> Html {
    html! {
        <li>
            <a href={props.item.url.clone()}>
                { props.item.name.clone() }
            </a>
        </li>
    }
}

#[function_component(ControlPlaylist)]
pub fn control_playlist(props: &PropsControlPlaylist) -> Html {

    let render_item = |item: &ContentPlaylist| {
        html! {
            <PlaylistItem item={item.clone()} />
        }
    };

    html! {
        <div class={classes!("control-playlist")}>
            <ul>
                { props.lists.iter().map(render_item).collect::<Html>() }
            </ul>
        </div>
    }
}