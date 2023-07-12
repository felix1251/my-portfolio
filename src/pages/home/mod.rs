use crate::components::molecules::home_main_section::HomeMainSection;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <HomeMainSection/>
        </>
    }
}
