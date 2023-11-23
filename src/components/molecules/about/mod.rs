use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
  html! {
    <div class="rounded-xl bg-dark-2 w-full drop-shadow-sm border border-dark-2 p-6 space-y-2">
      <h1 class="text-xl font-medium">{"About"}</h1>
      <p class="text-[15px] text-tertiary">
        {"Experienced Software Engineer with a demonstrated history of working in the information technology and services industry. Strong engineering professional with a Bachelor of Science in Computer Science focused in Software Engineering from Mindanao State University."}
      </p>
    </div>
  }
}
