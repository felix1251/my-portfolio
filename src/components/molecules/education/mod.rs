use crate::components::atoms::education_card::EducationCard;
use yew::prelude::*;

#[function_component(Education)]
pub fn education() -> Html {
  html! {
     <div class="rounded-xl bg-dark-2 w-full drop-shadow-sm border border-dark-2 p-6 space-y-1">
      <h1 class="text-xl font-medium">{"Education"}</h1>
      <div class="divide-y divide-tertiary/10">
        <EducationCard
          school={"Mindanao State University - Main Campus"}
          major={"Bachelor of Science in Computer Science, Major in Software Engineering"}
          duration={"Jan 2016 - Feb 2022"}
          img={"msu_logo.png"}
        />
      </div>
    </div>
  }
}
