use yew::prelude::*;
// use yew_router::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub highlight: String,
    pub go_to: String,
    pub go_to_text: String,
}

#[function_component(SectionHeader)]
pub fn section_header(props: &Props) -> Html {
    html! {
         <div class="flex flex-col space-y-4 lg:space-y-0 lg:flex-row justify-between items-start mb-12">
            <div class="basis-2/4">
                <span class="text-xs uppercase text-primary font-semibold mb-2 block">
                    {props.title.clone()}
                </span>
                <h2 class="text-4xl font-medium transition-colors duration-200 text-white dark:text-white">
                    {props.highlight.clone()}
                </h2>
            </div>
            <a class="text-tertiary hover:text-primary lg:basis-2/4 flex justify-end items-center space-x-1.5 cursor-pointer">
                <span class="text-xs uppercase font-semibold">
                    {props.go_to_text.clone()}
                </span>
                <Icon
                    class="w-3 h-3"
                    icon_id={IconId::FontAwesomeSolidArrowRight}
                />
            </a>
        </div>
    }
}
