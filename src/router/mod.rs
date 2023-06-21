use crate::pages::blogs::Blogs;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blogs")]
    Blogs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Blogs => html! { <Blogs/> },
        Route::NotFound => html! { <NotFound/> },
    }
}
