use crate::components::molecules::profile_card::ProfileCard;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <div class="space-y-5">
      <ProfileCard/>
    </div>
  }
}
