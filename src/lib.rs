mod components;
mod pages;

use components::molecules::custom_form::CustomForm;
use gloo::console::log;
use yew::prelude::*;

use crate::components::molecules::custom_form::Data;

#[function_component(App)]
pub fn app() -> Html {
    let form_submit = Callback::from(|data: Data| log!("user username: ", data.username));

    html! {
        <CustomForm onsubmit={form_submit}/>
    }
}
