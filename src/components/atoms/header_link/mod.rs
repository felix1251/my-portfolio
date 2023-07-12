use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub link: Route,
}

#[function_component(HeaderLink)]
pub fn header_link(props: &Props) -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    let is_active = if route == props.link {
        classes!("text-primary")
    } else {
        classes!("text-secondary", "dark:text-white")
    };

    html! {
        <Link<Route>
            classes={classes!("hidden md:block transition-colors duration-200".to_owned(), is_active)}
            to={props.link.clone()}
        >
            {props.children.clone()}
        </Link<Route>>
    }
}
