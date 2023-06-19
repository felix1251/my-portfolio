/*
* gloo for debugging
* yew for UI
* serde for handling objects
*/

// use gloo::console::log;
// use serde::{Deserialize, Serialize};
mod components;
use components::atoms::button::Button;
use yew::prelude::*;

// #[derive(Serialize, Deserialize)]
// struct  MyObject {
//     username: String
// }

#[function_component(App)]
pub fn app() -> Html {
    let class = "sample";
    let tasks = vec!["record", "play", "buying"];

    html! {
        <>
            <h1 class="text-red-500">{"Hello, world!"}</h1>
            if class == "app" {
                <p>{"hi there!"}</p>
            }else{
                <p>{"hi there! people"}</p>
            }

            <ul>
                {wrap_li(tasks)}
            </ul>

            <Button>{"Click me"}</Button>
        </>
    }
}

fn wrap_li(list: Vec<&str>) -> Vec<Html> {
    list.iter()
        .map(|item| {
            html! {
                <li>{item}</li>
            }
        })
        .collect()
}
