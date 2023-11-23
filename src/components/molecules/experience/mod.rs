use crate::components::atoms::xp_card::XpCard;
use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
  html! {
    <div class="rounded-xl bg-dark-2 w-full drop-shadow-sm border border-dark-2 p-6 space-y-1">
      <h1 class="text-xl font-medium">{"Experience"}</h1>
      <div class="divide-y divide-tertiary/10">
        <XpCard
          role={"Software Engineer"}
          company={"MyntExchange"}
          duration={"Aug 2023 - Present"}
          location={"Iceland · Remote"}
          img={"mynt_logo.jpeg"}
        />
        <XpCard
          role={"Full Stack Developer"}
          company={"Black Ink Tech · Full-time"}
          duration={"Aug 2023 - Nov 2023 · 4 mos"}
          location={"South Carolina, United States · Remote"}
          img={"bit_logo.jpeg"}
        />
        <XpCard
          role={"Full Stack Developer"}
          company={"Montani International Inc. · Full-time"}
          duration={"Jun 2023 - Aug 2023 · 3 mos"}
          location={"Makati, National Capital Region, Philippines · Remote"}
          img={"montani_logo.jpeg"}
        />
        <XpCard
          role={"Software Developer"}
          company={"MegaXcess IT Solutions, Inc. · Full-time"}
          duration={"Jan 2023 - Jun 2023 · 6 mos"}
          location={"Ortigas Center, Pasig, National Capital Region, Philippines · Hybrid"}
          img={"megaxcess_logo.jpg"}
        />
        <XpCard
          role={"Back End Developer"}
          company={"iRipple, Inc. · Full-time"}
          duration={"Nov 2021 - Dec 2022 · 1 yr 2 mos"}
          location={"Pasig, National Capital Region, Philippines · Remote"}
          img={"irriple_logo.jpeg"}
        />
      </div>
    </div>
  }
}
