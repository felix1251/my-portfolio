use crate::components::atoms::logo_text::LogoText;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
  html! {
    <header class="w-full">
      <div class="mx-auto max-w-4xl px-6 md:px-8 flex py-6 items-center justify-between">
        <LogoText />
      </div>
    </header>
  }
}
