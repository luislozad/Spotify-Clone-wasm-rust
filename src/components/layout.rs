use yew_router::prelude::*;
use yew::prelude::*;

use crate::components::prelude::*;
use crate::components::grid::{Grid, Row, Column};
use crate::types::*;

#[function_component(Layout)]
pub fn layout() -> Html {
    let AppContext { count, .. } = use_context::<AppContext>().expect("no ctx found");

    html! {
        <BrowserRouter>
            <Grid columns={WidthColumn::Equal}>
                <Column width={15} padded={false}>
                    <UserContent />
                </Column>
                <Column width={50} padded={false}>
                    <UserPage />
                </Column>
            </Grid>
        </BrowserRouter>
    }
}