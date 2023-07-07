use crate::components::templates::primary_layout::PrimaryLayout;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <PrimaryLayout>
            <h1 class={classes!("text-secondary")}>
                {"This is Home page"}
            </h1>
            <p class="text-orange-500">{"Me felix"}</p>
        </PrimaryLayout>
    }
}
