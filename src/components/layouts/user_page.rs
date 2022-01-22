use yew_router::prelude::*;
use yew::prelude::*;

use crate::types::router::*;

#[function_component(UserPage)]
pub fn user_page() -> Html {

    html! {
        <>
            {"Pagina"}
            <Switch<Route> render={Switch::render(switch)} />
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Other => html! { <h1>{ "Other" }</h1> },
    }
}