use yew::prelude::*;

use crate::components::prelude::*;
use crate::types::*;

#[function_component(UserList)]
pub fn user_list() -> Html {
    let lists = vec![
        ContentPlaylist{ name: "Nueva playlist".to_string(), url: "nueva_playlist".to_string() },
        ContentPlaylist{ name: "Regueton la nueva fake".to_string(), url: "regg_fake".to_string() },
        ContentPlaylist{ name: "Pop la nueva generacion".to_string(), url: "pop_gen".to_string() },
        ContentPlaylist{ name: "Otros".to_string(), url: "otros".to_string() },
    ];

    html! {
        <div>
            {"List user"}
            <ControlList />
            <ControlPlaylist {lists} />
        </div>
    }
}