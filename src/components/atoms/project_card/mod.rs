use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub context: String,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &Props) -> Html {
    html! {
        <div class="p-6 rounded-2xl bg-quaternary">
            <div class="h-[200px] w-full bg-white relative overflow-hidden mb-3 rounded-xl">
            </div>
            <div class="font-semibold text-xl mb-2">
                { props.title.clone() }
            </div>
            <div class="text-md text-tertiary">
                { props.context.clone() }
            </div>
        </div>
    }
}
