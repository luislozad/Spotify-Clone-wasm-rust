use yew::prelude::*;
use std::rc::*;

use crate::lang::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct AppContextProps {
    pub count: i32,
    pub lang: Lang,
}

impl Reducible for AppContextProps {
    type Action = AppContextProps;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppContextProps { 
            count: action.count, 
            lang: action.lang, 
        }.into()
    }
}

pub type AppContext = UseReducerHandle<AppContextProps>;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsAppContextProvider {
    pub children: Children,
}

#[function_component(AppContextProvider)]
pub fn app_context_provider(props: &PropsAppContextProvider) -> Html {
    let ctx = use_reducer(|| AppContextProps {
        count: 1,
        lang: LangEs::new(),
    });

    html! {
        <ContextProvider<AppContext> context={ctx}>
            { props.children.clone() }
        </ContextProvider<AppContext>>
    }
}