use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or("button".to_owned())]
    pub html_type: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <button
            class="bg-blue-500 px-3.5 py-2 rounded-sm text-white"
            type={props.html_type.clone()}
        >
            { for props.children.iter() }
        </button>
    }
}
