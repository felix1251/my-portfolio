use crate::components::organisms::footer::Footer;
use crate::components::organisms::header::Header;
use crate::state::State;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
    let (state, _) = use_store::<State>();
    let theme_class = classes!(state.theme.clone());

    html! {
        <div id="__yew" class={theme_class}>
            <div class="transition-colors duration-200 dark:bg-dark h-screen flex flex-col">
                <Header/>
                <main class="flex-1 w-full">
                    { props.children.clone() }
                </main>
                <Footer/>
            </div>
        </div>
    }
}
