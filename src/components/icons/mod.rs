use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct PropsIcon {
    #[prop_or(false)]
    pub active: bool,
}

#[function_component(HomeIcon)]
pub fn home_icon(props: &PropsIcon) -> Html {
    html! {
        if props.active {
            <svg class={classes!("icon-color")} role="img" height="24" width="24" viewBox="0 0 24 24">
                <path d="M21 22V7.174l-9.001-5.195L3 7.214V22h7v-7h4v7z"></path>
            </svg>
        } else {
            <svg class={classes!("icon-color")} role="img" height="24" width="24" viewBox="0 0 24 24">
                <path d="M9 14h6v7h5V7.8l-8-4.6-8 4.6V21h5v-7zm1 8H3V7.2L12 2l9 5.2V22h-7v-7h-4v7z"></path>
            </svg>
        }
    }
}

#[function_component(SearchIcon)]
pub fn search_icon(props: &PropsIcon) -> Html {
    html! {
        if props.active {
            <svg class={classes!("icon-color")} role="img" height="24" width="24" viewBox="0 0 24 24">
                <path d="M16.736 16.262A8.457 8.457 0 0019 10.5a8.5 8.5 0 10-3.779 7.067l4.424 5.18 1.521-1.299-4.43-5.186zM10.5 17C6.916 17 4 14.084 4 10.5S6.916 4 10.5 4 17 6.916 17 10.5 14.084 17 10.5 17z"></path>
            </svg>
        } else {
            <svg class={classes!("icon-color")} role="img" height="24" width="24" viewBox="0 0 24 24">
                <path d="M16.387 16.623A8.47 8.47 0 0019 10.5a8.5 8.5 0 10-8.5 8.5 8.454 8.454 0 005.125-1.73l4.401 5.153.76-.649-4.399-5.151zM10.5 18C6.364 18 3 14.636 3 10.5S6.364 3 10.5 3 18 6.364 18 10.5 14.636 18 10.5 18z"></path>
            </svg>
        }
    }
}

#[function_component(CollectionIcon)]
pub fn collection_icon(props: &PropsIcon) -> Html {
    html! {
        if props.active {
            <svg class={classes!("icon-color")} role="img" height="24" width="24" viewBox="0 0 24 24"><path d="M14.617 3.893l-1.827.814 7.797 17.513 1.827-.813-7.797-17.514zM3 22h2V4H3v18zm5 0h2V4H8v18z"></path></svg>
        } else {
            <svg class={classes!("icon-color")} role="img" height="24" width="24" viewBox="0 0 24 24"><path d="M13.66 4.097l-.913.406 7.797 17.513.914-.406L13.66 4.097zM3 22h1V4H3v18zm6 0h1V4H9v18z"></path></svg>
        }
    }
}

#[function_component(PlusSquareIcon)]
pub fn plus_icon() -> Html {
    html! {
        <svg class={classes!("icon-color")} role="img" height="12" width="12" aria-hidden="true" viewBox="0 0 16 16" ><path d="M14 7H9V2H7v5H2v2h5v5h2V9h5z"></path><path fill="none" d="M0 0h16v16H0z"></path></svg>
    }
}

#[function_component(LikeSquareIcon)]
pub fn like_square_icon() -> Html {
    html! {
        <svg class={classes!("icon-color")} role="img" height="12" width="12" aria-hidden="true" viewBox="0 0 16 16" ><path fill="none" d="M0 0h16v16H0z"></path><path d="M13.797 2.727a4.057 4.057 0 00-5.488-.253.558.558 0 01-.31.112.531.531 0 01-.311-.112 4.054 4.054 0 00-5.487.253c-.77.77-1.194 1.794-1.194 2.883s.424 2.113 1.168 2.855l4.462 5.223a1.791 1.791 0 002.726 0l4.435-5.195a4.052 4.052 0 001.195-2.883 4.057 4.057 0 00-1.196-2.883z"></path></svg>
    }
}