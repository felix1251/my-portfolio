mod components;
mod pages;
mod router;
use components::templates::primary_layout::PrimaryLayout;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <PrimaryLayout>
                <Switch<Route> render={switch} />
            </PrimaryLayout>
        </BrowserRouter>
    }
}
