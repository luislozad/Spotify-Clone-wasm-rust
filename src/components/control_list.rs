use yew::prelude::*;

use crate::components::prelude::*;
use crate::types::*;
use crate::components::grid::{Grid, Row, Column};
use crate::components::icons::{PlusSquareIcon, LikeSquareIcon};

#[function_component(ControlList)]
pub fn control_list() -> Html {
    let AppContextProps { ref lang, .. } = *use_context::<AppContext>().expect("no AppContext found");    

    let padded = false;
    let width_icon = 10;
    let width_label = 100;

    html! {
        <div>
            <Grid>
                <Row>
                    <button>
                        <Grid columns={WidthColumn::EqualPx}>
                            <Column {padded} width={width_icon}>
                                <PlusSquareIcon />
                            </Column>
                            <Column {padded} width={width_label}>
                                <span>{lang.create_playlist.clone()}</span>
                            </Column>
                        </Grid>
                    </button>
                </Row>
                <Row>
                    <button>
                        <Grid columns={WidthColumn::EqualPx}>
                            <Column {padded} width={width_icon}>
                                <LikeSquareIcon />
                            </Column>
                            <Column {padded} width={width_label}>
                                <span>{lang.liked_songs.clone()}</span>
                            </Column>
                        </Grid>
                    </button>
                </Row>
            </Grid>
        </div>
    }
}