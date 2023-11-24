use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub school: String,
  pub major: String,
  pub duration: String,
  pub img: String,
}

#[function_component(EducationCard)]
pub fn education_card(props: &Props) -> Html {
  html! {
    <div class="flex gap-3 py-4">
      <img class="bg-tertiary w-[52px] h-[52px] rounded-md object-cover object-center" src={format!("assets/img/{}", props.img.clone())} alt="ex-1"/>
      <div class="flex flex-col">
        <span class="font-medium text-gray-50">{props.school.clone()}</span>
        <span class="text-[15px] text-gray-300">{props.major.clone()}</span>
        <span class="text-tertiary text-[15px] ">{props.duration.clone()}</span>
      </div>
    </div>
  }
}
