use yew::prelude::*;

use crate::components::prelude::*;
use crate::components::grid::{Grid, Row};

#[function_component(UserContent)]
pub fn user_content() -> Html {
    html! {
        <div class={classes!("pt-6", "bg-black", "text-color-second")}>
            <Container>
                <Grid>
                    <Row>
                        <Logo />
                    </Row>
                    <Row>
                        <Nav />
                    </Row>
                    <Row>
                        <UserList />
                    </Row>
                </Grid>
            </Container>
        </div>
    }
}