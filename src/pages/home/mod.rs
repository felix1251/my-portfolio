use crate::components::molecules::home_main_section::HomeMainSection;
use crate::constants::PROJECTS;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <HomeMainSection/>
            {
                PROJECTS.into_iter().map(|project| {
                    html!{
                        <div>{ format!("Hello, I'am {}!", project.title) }</div>
                    }
                }).collect::<Html>()
            }
        </>
    }
}
