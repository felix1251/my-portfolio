use crate::components::molecules::about::About;
use crate::components::molecules::education::Education;
use crate::components::molecules::experience::Experience;
use crate::components::molecules::profile::Profile;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <div class="space-y-5">
      <Profile/>
      <About/>
      <Experience/>
      <Education/>
    </div>
  }
}
