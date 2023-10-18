use crate::components::atoms::card::Card;
use crate::components::atoms::section_header::SectionHeader;
use crate::constants::PROJECTS;
use yew::prelude::*;

#[function_component(HomeProjectSection)]
pub fn home_project_section() -> Html {
    html! {
        <section class="mx-auto max-w-[85rem] px-6 md:px-8 py-14">
            <SectionHeader
                title="Some projects I worked on."
                highlight="Building awesome projects for awesome people."
                go_to="/"
                go_to_text="Browse more"
            />
            <div class= "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10">
                {
                    PROJECTS.into_iter().take(8).map(|project| {
                        html!{
                            <Card
                                title={project.title}
                                context={project.context}
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}
