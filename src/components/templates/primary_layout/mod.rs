use crate::components::organisms::footer::Footer;
use crate::components::organisms::header::Header;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
    html! {
        <div class="transition-colors duration-200 dark:bg-dark h-screen overflow-auto flex flex-col">
            <Header/>
            <main class="flex-1 w-full">
                { props.children.clone() }
            </main>
            <Footer/>
        </div>
    }
}
