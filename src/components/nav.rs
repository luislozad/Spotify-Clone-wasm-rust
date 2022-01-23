use yew::prelude::*;

use crate::components::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let ctx = use_context::<AppContext>().expect("no ctx found");
    html! {
        <ul class={classes!("text-white")}>
            <li>
                <span>{ctx.lang.nav_home.clone()}</span>
            </li>
            <li>
                <span>{ctx.lang.nav_search.clone()}</span>
            </li>
            <li>
                <span>{ctx.lang.nav_collection.clone()}</span>
            </li>
        </ul>
    }
}