use crate::components::molecules::home_main_section::HomeMainSection;
use crate::components::molecules::home_project_section::HomeProjectSection;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <HomeMainSection/>
            <HomeProjectSection/>
        </>
    }
}
