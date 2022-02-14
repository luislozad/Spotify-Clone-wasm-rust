use yew::prelude::*;
use web_sys::MouseEvent;

use crate::components::prelude::*;
use crate::components::grid::{Grid, Column};
use crate::components::icons::*;
use crate::types::*;

#[derive(Default)]
struct Active {
    home: bool,
    search: bool,
    collection: bool,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let AppContextProps{ ref lang, .. } = *use_context::<AppContext>().expect("no AppContext found");

    let padded = false;
    let width_column_default = WidthColumn::EqualPx;
    let width_icon = 10;
    let width_label = 100;

    let nav_active = use_state(|| Active::default());

    let class_active_home = if nav_active.home {
        "text-white fill-white"
    } else {
        ""
    };

    let class_active_search = if nav_active.search {
        "text-white fill-white"
    } else {
        ""
    };

    let class_active_collection = if nav_active.collection {
        "text-white fill-white"
    } else {
        ""
    };

    let on_click_home = {
        let state = nav_active.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            state.set(Active { home: true, ..Default::default() });
        }
    };

    let on_click_search = {
        let state = nav_active.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            state.set(Active { search: true, ..Default::default() });
        }
    };

    let on_click_collection = {
        let state = nav_active.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            state.set(Active { collection: true, ..Default::default() });
        }
    };

    html! {
        <ul class={classes!("w-full", "user-nav")}>
            <li class={classes!("cursor-pointer", "mb-3", class_active_home)}>
                <a onclick={on_click_home} href="/">
                    <Grid columns={width_column_default.clone()}>
                        <Column {padded} width={width_icon}>
                            <HomeIcon active={nav_active.home} />
                        </Column>
                        <Column {padded} width={width_label}>
                            <span>{lang.nav_home.clone()}</span>
                        </Column>
                    </Grid>
                </a>
            </li>
            <li class={classes!("cursor-pointer", "mb-3", class_active_search)}>
                <a onclick={on_click_search} href="/search">
                    <Grid columns={width_column_default.clone()}>
                        <Column {padded} width={width_icon}>
                            <SearchIcon active={nav_active.search} />
                        </Column>
                        <Column {padded} width={width_label}>
                            <span>{lang.nav_search.clone()}</span>
                        </Column>
                    </Grid>            
                </a>
            </li>
            <li class={classes!("cursor-pointer", class_active_collection)}>
                <a onclick={on_click_collection} href="/collection">
                    <Grid columns={width_column_default.clone()}>
                        <Column {padded} width={width_icon}>
                            <CollectionIcon active={nav_active.collection} />
                        </Column>
                        <Column {padded} width={width_label}>
                            <span>{lang.nav_collection.clone()}</span>
                        </Column>
                    </Grid>              
                </a>
            </li>
        </ul>
    }
}