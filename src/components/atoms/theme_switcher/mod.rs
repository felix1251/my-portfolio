use crate::state::State;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::*;

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let (state, dispatch) = use_store::<State>();

    let onclick = Callback::from(move |_: MouseEvent| {
        let theme = state.theme.clone();
        let dispatch = dispatch.clone();

        if theme == "dark" {
            dispatch.reduce_mut(|store| store.theme = "light".to_string());
        } else {
            dispatch.reduce_mut(|store| store.theme = "dark".to_string());
        }
    });

    html! {
        <button {onclick} class="w-10 rounded-full overflow-hidden p-0.5 bg-primary">
            <div class="relative w-5 h-5 flex justify-center items-center rounded-full bg-white dark:bg-dark transition-margin duration-200 ml-0 dark:ml-4">
                <Icon
                    class="absolute w-3.5 h-3.5 text-secondary transition-all duration-200 dark:text-white opacity-0 dark:opacity-100"
                    icon_id={IconId::FontAwesomeSolidSun}
                />
                <Icon
                    class="absolute w-3.5 h-3.5 text-secondary transition-all duration-200 dark:text-white opacity-100 dark:opacity-0"
                    icon_id={IconId::BootstrapMoonFill}
                />
            </div>
        </button>
    }
}
