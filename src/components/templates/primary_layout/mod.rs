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
    let theme = state.theme.clone();
    let class_name = classes!(theme);

    html! {
        <div id="__yew" class={class_name}>
            <div class="transition-colors duration-200 dark:bg-dark h-screen">
                <Header/>
                <main class="mx-auto max-w-[85rem] px-6 md:px-8 py-2 md:py-3">
                    { props.children.clone() }
                </main>
            </div>
        </div>
    }
}
