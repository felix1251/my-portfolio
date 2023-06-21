mod components;
mod pages;
mod router;

// use gloo::console::log;
use components::molecules::custom_form::CustomForm;
use components::molecules::custom_form::Data;
use router::{switch, Route};
use std::ops::Deref;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub fullname: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.fullname = data.fullname;
            user_state.set(user);
        })
    };

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <CustomForm onsubmit={form_submit}/>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<User>>
    }
}
