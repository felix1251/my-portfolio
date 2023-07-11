use crate::components::organisms::header::Header;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
    html! {
        <>
            <Header/>
            <main class="mx-auto max-w-[87rem] px-6 md:px-8 py-2 md:py-3">
                { props.children.clone() }
            </main>
        </>
    }
}
