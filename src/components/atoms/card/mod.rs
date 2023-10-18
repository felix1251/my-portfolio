use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub context: String,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <div class="p-6 rounded-2xl bg-dark-2">
            <div class="relative h-[200px] duration-200 bg-dark relative overflow-hidden mb-3 rounded-xl">
            </div>
            <div class="font-semibold text-lg mb-2 transition-colors duration-200 text-white">
                { props.title.clone() }
            </div>
            <div class="text-md text-tertiary">
                { props.context.clone() }
            </div>
        </div>
    }
}
