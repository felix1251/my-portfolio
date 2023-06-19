mod components;

use components::atoms::button::Button;
use components::atoms::input_text::InputText;
use gloo::console::log;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let username_onchanged =
        Callback::from(|username| log!("your usernamne change to: ", username));

    html! {
        <form class="flex flex-col max-w-md gap-2 rounded-sm p-2">
            <InputText
                name="username"
                label="Username"
                handle_onchange={username_onchanged}
            />
            <Button html_type="submit">{"Click me"}</Button>
        </form>
    }
}
