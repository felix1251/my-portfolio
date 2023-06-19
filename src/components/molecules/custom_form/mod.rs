use crate::components::atoms::button::Button;
use crate::components::atoms::input_text::InputText;
// use gloo::console::log;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub fullname: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_onchanged = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data)
    });

    let cloned_state = state.clone();
    let fullname_onchanged = Callback::from(move |fullname| {
        let mut data = cloned_state.deref().clone();
        data.fullname = fullname;
        cloned_state.set(data)
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data)
    });

    html! {
        <form onsubmit={onsubmit} class="flex flex-col max-w-md gap-2 rounded-sm p-2">
            <InputText
                name="username"
                label="Username"
                handle_onchange={username_onchanged}
            />
            <InputText
                name="fullname"
                label="Fullname"
                handle_onchange={fullname_onchanged}
            />
            <Button
                html_type="submit"
            >
                {"Click me"}
            </Button>
        </form>
    }
}
