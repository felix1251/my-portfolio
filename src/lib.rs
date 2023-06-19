/*
* gloo for debugging
* yew for UI
* serde for handling objects
*/
use gloo::console::log;
use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct  MyObject {
    username: String
}

#[function_component(App)]
pub fn app() -> Html {
    let name="brooks";
    let my_object = MyObject {
        username: name.to_owned(),
    };

    log!(serde_json::to_string_pretty(&my_object).unwrap());
    html! {
        <h1>{"Hello, world!"}</h1>
    }
}
