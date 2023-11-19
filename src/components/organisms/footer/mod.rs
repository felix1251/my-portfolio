use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Footer)]
pub fn footer() -> Html {
  html! {
    <footer class="space-y-3 w-full mx-auto max-w-4xl px-6 md:px-8 items-center flex justify-between">
      <div class="text-[15px] text-tertiary sm:text-center font-medium">
          {"© 2023 Felix's Pages"}
      </div>
      <div class="flex space-x-4 sm:justify-center md:mt-0">
          <Icon class="text-tertiary hover:text-primary" icon_id={IconId::BootstrapGithub}/>
      </div>
    </footer>
  }
}
