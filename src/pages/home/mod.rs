use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <h1 class={classes!("text-secondary")}>
            {"This is Home page"}
        </h1>
    }
}
