use crate::components::atoms::header_link::HeaderLink;
use crate::components::atoms::logo_text::LogoText;
use crate::router::Route;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="w-full border-b border-dark-2">
            <div class="mx-auto max-w-[85rem] px-6 md:px-8 flex h-[5rem] items-center justify-between">
                <LogoText />
                <div class="flex gap-6 text-[16px] primary-text-color font-medium items-center">
                    <HeaderLink link={Route::Home}>{"Home"}</HeaderLink>
                    <HeaderLink link={Route::Projects}>{"Projects"}</HeaderLink>
                    <HeaderLink link={Route::Blogs}>{"Blogs"}</HeaderLink>
                    <HeaderLink link={Route::About}>{"About"}</HeaderLink>
                </div>
            </div>
        </header>
    }
}
