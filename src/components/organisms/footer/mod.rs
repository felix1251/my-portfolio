use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
  html! {
    <footer class="space-y-3 w-full mx-auto max-w-4xl px-6 md:px-8 items-center flex justify-between">
      <div class="text-[15px] text-tertiary sm:text-center font-medium">
          {"© 2023 Felix's Pages"}
      </div>
    </footer>
  }
}
