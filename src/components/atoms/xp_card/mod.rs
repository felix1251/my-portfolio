use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub role: String,
  pub company: String,
  pub duration: String,
  pub location: String,
  pub img: String,
}

#[function_component(XpCard)]
pub fn xp_card(props: &Props) -> Html {
  html! {
    <div class="flex gap-3 py-4">
      <img class="bg-tertiary w-14 h-14 rounded-md object-cover object-center" src={format!("assets/img/{}", props.img.clone())} alt="ex-1"/>
      <div class="flex flex-col">
        <span class="font-medium text-gray-50">{props.role.clone()}</span>
        <span class="text-[15px] text-gray-300">{props.company.clone()}</span>
        <span class="text-tertiary text-[15px] ">{props.duration.clone()}</span>
        <span class="text-tertiary text-[15px] ">{props.location.clone()}</span>
      </div>
    </div>
  }
}
