use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::projects::Projects;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/projects")]
  Projects,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(route: Route) -> Html {
  match route {
    Route::Home => html! { <Home/> },
    Route::Projects => html! { <Projects/> },
    Route::NotFound => html! { <NotFound/> },
  }
}
