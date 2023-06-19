use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
    #[prop_or("button".to_string())]
    pub html_type: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <button type={props.html_type.clone()} class="bg-blue-500 px-3.5 py-2 rounded-sm text-white">
            { for props.children.iter() }
        </button>
    }
}
