use crate::components::organisms::footer::Footer;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
  html! {
    <>
      <main class="margin-max-screen py-3 md:py-5">
        { props.children.clone() }
      </main>
      <Footer/>
    </>
  }
}
