use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <div class="space-y-5">
      <div class="rounded-xl bg-dark-2 w-full drop-shadow-sm border border-dark-2">
        <div class="h-52 w-full bg-cover bg-center rounded-t-xl" style="background-image: url('assets/img/cover.jpeg');"></div>
        <div class="px-6 py-4">
          <img class="border-[6px] border-dark-2 -mt-28 w-44 h-44 rounded-full" src="assets/img/profile.jpeg" alt="cover"/>
          <div class="flex gap-1.5 items-center mt-0.5">
            <h1 class="text-[1.68rem]">{"Felix Abacajen"}</h1>
            <div class="text-[15px] text-tertiary">{"(He/Him)"}</div>
          </div>
          <div class="space-y-2.5">
            <div class="text-[17px] text-gray-300">
              {"Software Engineer | Go | Rust | Ruby | TS | JS"}
            </div>
            <div class="flex gap-3">
              <Icon
                class="text-tertiary hover:text-primary"
                width={"1.6em".to_owned()}
                height={"1.6em".to_owned()}
                icon_id={IconId::BootstrapLinkedin}
              />
              <Icon
                class="text-tertiary hover:text-primary"
                width={"1.6em".to_owned()}
                height={"1.6em".to_owned()}
                icon_id={IconId::BootstrapGithub}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  }
}
