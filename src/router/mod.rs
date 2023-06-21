use crate::pages::blogs::Blogs;
use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blogs")]
    Blogs,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/>},
        Route::Blogs => html! { <Blogs/>},
    }
}
