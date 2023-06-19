use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <button class="bg-blue-500 px-3.5 py-2 rounded-sm text-white">
            { for props.children.iter() }
        </button>
    }
}
