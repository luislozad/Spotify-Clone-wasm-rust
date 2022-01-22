use yew::prelude::*;

use crate::helpers::*;
use crate::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct GridContext {
    pub columns: WidthColumn,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PropsGrid {
    pub children: Children,
    #[prop_or(WidthColumn::Relative)]
    pub columns: WidthColumn,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PropsRow {
    pub children: Children
}

#[derive(Properties, PartialEq, Clone)]
pub struct PropsColumn {
    pub children: Children,
    #[prop_or(0)]
    pub width: u8,
    #[prop_or(true)]
    pub padded: bool,
}

#[function_component(Grid)]
pub fn grid(props: &PropsGrid) -> Html {
    let ctx = use_state(|| GridContext {
        columns: props.columns.clone(),
    });

    html! {
        <ContextProvider<GridContext> context={(*ctx).clone()}>
            <div class={classes!("flex", "flex-row", "flex-wrap", "items-stretch")}>
                { props.children.clone() }
            </div>
        </ContextProvider<GridContext>>
    }
}

#[function_component(Row)]
pub fn row(props: &PropsRow) -> Html {
    html! {
        <div class={classes!("flex", "flex-wrap", "flex-row", "w-full", "py-3.5", "items-stretch")}>
            { props.children.clone() }
        </div>
    }
}

#[function_component(Column)]
pub fn column(props: &PropsColumn) -> Html {
    let GridContext { columns, .. } = use_context::<GridContext>().expect("no GridContext found");

    let class_width = get_class_width_column(columns.clone(), props.width);
    let class_padding = {
        if props.padded {
            "px-4"
        } else {
            ""
        }
    };

    html! {
        <div class={classes!(class_padding, "inline-block", class_width)}>
            { props.children.clone() }
        </div>
    }
}
