use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
    html! {
        <main>
            <div class="flex gap-2">
                <Link<Route> to={Route::Home}>{"Home route"}</Link<Route>>
                <Link<Route> to={Route::Blogs}>{"Blogs route"}</Link<Route>>
            </div>
            { props.children.clone() }
        </main>
    }
}
