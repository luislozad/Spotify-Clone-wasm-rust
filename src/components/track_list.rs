use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsTrackList {
    list: Vec<String>
}

#[function_component(TrackList)]
pub fn track_list(props: &PropsTrackList) -> Html {
    html! {
        <div>
            {"Track List"}
        </div>
    }
}