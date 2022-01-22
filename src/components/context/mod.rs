use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub count: i32,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PropsAppContextProvider {
    pub children: Children,
}

#[function_component(AppContextProvider)]
pub fn app_context_provider(props: &PropsAppContextProvider) -> Html {
    let ctx = use_state(|| AppContext {
        count: 1
    });

    html! {
        <ContextProvider<AppContext> context={(*ctx).clone()}>
            { props.children.clone() }
        </ContextProvider<AppContext>>
    }
}