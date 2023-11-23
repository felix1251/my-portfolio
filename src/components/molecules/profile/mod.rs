use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Profile)]
pub fn profile() -> Html {
  html! {
   <div class="rounded-xl bg-dark-2 w-full drop-shadow-sm border border-dark-2">
      <div class="h-52 w-full bg-cover bg-center rounded-t-xl" style="background-image: url('assets/img/cover.jpeg');"></div>
      <div class="p-6">
        <img class="ring-[5px] ring-dark-2 bg-tertiary -mt-28 w-44 h-44 rounded-full" src="assets/img/profile.jpeg" alt="profile"/>
        <div class="flex gap-1.5 items-center mt-2">
          <h1 class="text-[1.68rem]">{"Felix Abacajen"}</h1>
          <div class="text-[15px] text-tertiary">{"(He/Him)"}</div>
        </div>
        <div class="space-y-5">
          <div>
            <div class="text-[17px] text-gray-300">
              {"Software Engineer | Go | Rust | Ruby | TS | JS"}
            </div>
            <p class="text-tertiary text-[15px]">{"Kitcharao, Caraga, Philippines"}</p>
          </div>
          <div class="flex gap-3">
            <Icon
              class="text-tertiary hover:text-primary"
              width={"1.5em".to_owned()}
              height={"1.5em".to_owned()}
              icon_id={IconId::BootstrapGithub}
            />
            <Icon
              class="text-tertiary hover:text-primary"
              width={"1.5em".to_owned()}
              height={"1.5em".to_owned()}
              icon_id={IconId::BootstrapLinkedin}
            />
          </div>
        </div>
      </div>
    </div>
  }
}
