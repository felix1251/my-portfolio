use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(InputText)]
pub fn input_text(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event| {
        let input = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        log!(input);
    });

    html! {
        <input type="text" name={props.name.clone()} onchange={onchange}/>
    }
}
