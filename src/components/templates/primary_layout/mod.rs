use crate::components::organisms::header::Header;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[derive(PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local")]
pub struct State {
    pub theme: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
        }
    }
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
    let (state, dispatch) = use_store::<State>();
    let theme = state.theme.clone();
    let class_name = classes!(theme);

    let onclick = Callback::from(move |_: MouseEvent| {
        let theme = state.theme.clone();
        let dispatch = dispatch.clone();

        if theme == "dark" {
            dispatch.reduce_mut(|store| store.theme = "light".to_string());
        } else {
            dispatch.reduce_mut(|store| store.theme = "dark".to_string());
        }
    });

    html! {
        <div id="__yew" class={classes!(class_name)}>
            <div class="transition-colors duration-200 dark:bg-dark h-screen">
                <Header/>
                <main class="mx-auto max-w-[85rem] px-6 md:px-8 py-2 md:py-3">
                    { props.children.clone() }
                    <button class="p-2 bg-primary" {onclick}>{"click"}</button>
                </main>
            </div>
        </div>
    }
}
