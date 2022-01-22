use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsContainer {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &PropsContainer) -> Html {
    html! {
        <div class={classes!("container", "px-8")}>
            { props.children.clone() }
        </div>
    }
}