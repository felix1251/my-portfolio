use crate::components::organisms::footer::Footer;
use crate::components::organisms::header::Header;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component(PrimaryLayout)]
pub fn primary_layout(props: &Props) -> Html {
  html! {
    <>
      <Header/>
      <main class="mx-auto max-w-4xl px-6 md:px-8 py-5">
        { props.children.clone() }
      </main>
      <Footer/>
    </>
  }
}
