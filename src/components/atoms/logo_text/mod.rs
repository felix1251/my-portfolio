use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LogoText)]
pub fn logo_text() -> Html {
  html! {
    <Link<Route>
      to={Route::Home}
      classes={classes!("font-medium", "text-[18px]", "text-white", "transition-colors", "duration-200")}
    >
      {"felixabacajen"}
      <span class="text-primary">{".com"}</span>
    </Link<Route>>
  }
}
