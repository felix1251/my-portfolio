use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! {
    <h1>{"404 not found"}</h1>
  }
}
