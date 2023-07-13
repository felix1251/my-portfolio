use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="w-full py-10 mx-auto max-w-[85rem] px-6 md:px-8 md:flex md:items-center md:justify-between">
            <span class="text-[15px] text-tertiary sm:text-center font-medium">
                {"© 2023 Felix's Pages"}
            </span>
            <div class="flex space-x-4 sm:justify-center md:mt-0">
                <Icon class="text-tertiary hover:text-primary" icon_id={IconId::BootstrapGithub}/>
            </div>
        </footer>
    }
}
