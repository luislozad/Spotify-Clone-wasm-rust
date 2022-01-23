use yew::prelude::*;

mod components;
mod types;
mod helpers;
mod lang;

use crate::components::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <AppContextProvider>
            <Layout />
        </AppContextProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}