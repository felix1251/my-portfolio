mod components;
mod constants;
mod pages;
mod router;
mod state;

use components::templates::primary_layout::PrimaryLayout;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
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

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);

    return html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    };
}
