use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsButtonPlay {
    pub onclick: Callback<()>
}

#[function_component(ButtonPlay)]
pub fn btn_play(props: &PropsButtonPlay) -> Html {
    let onclick = {
        let handle = props.onclick.clone();
        move |_| handle.emit(())
    };
    html! {
        <button {onclick}>{">"}</button>
    }
}