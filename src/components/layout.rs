use yew_router::prelude::*;
use yew::prelude::*;

use crate::components::prelude::*;
use crate::components::grid::{Grid, Row, Column};
use crate::types::*;

#[function_component(Layout)]
pub fn layout() -> Html {
    html! {
        <BrowserRouter>
            <Grid columns={WidthColumn::EqualPx}>
                <Column width={60} padded={false}>
                    <UserContent />
                </Column>
                <Column width={100} padded={false}>
                    <UserPage />
                </Column>
            </Grid>
        </BrowserRouter>
    }
}