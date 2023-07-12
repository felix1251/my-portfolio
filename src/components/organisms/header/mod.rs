use crate::components::atoms::header_link::HeaderLink;
use crate::components::atoms::theme_switcher::ThemeSwitcher;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="w-full border-b transition-colors dark:border-gray-700 duration-200">
            <div class="mx-auto max-w-[85rem] px-6 md:px-8 flex h-[4.8rem] items-center justify-between">
                <Link<Route> classes="font-medium text-secondary dark:text-white transition-colors duration-200 text-[18px] cursor-pointer" to={Route::Home}>
                    {"felixabacajen"}
                    <span class="text-primary">{".com"}</span>
                </Link<Route>>
                <div class="flex gap-6 text-[16px] primary-text-color uppercase font-medium items-center">
                    <HeaderLink link={Route::Home}>{"Porfolio"}</HeaderLink>
                    <HeaderLink link={Route::Projects}>{"Projects"}</HeaderLink>
                    <HeaderLink link={Route::Blogs}>{"Blogs"}</HeaderLink>
                    <HeaderLink link={Route::About}>{"About"}</HeaderLink>
                    <ThemeSwitcher/>
                </div>
            </div>
        </header>
    }
}
