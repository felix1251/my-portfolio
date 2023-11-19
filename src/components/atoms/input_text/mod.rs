use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub name: String,
  pub handle_onchange: Callback<String>,
  #[prop_or("".to_owned())]
  pub placeholder: String,
  #[prop_or("".to_owned())]
  pub label: String,
}

#[function_component(InputText)]
pub fn input_text(props: &Props) -> Html {
  let handle_onchange = props.handle_onchange.clone();
  let onchange = Callback::from(move |event: Event| {
    let value = event
      .target()
      .unwrap()
      .unchecked_into::<HtmlInputElement>()
      .value();
    handle_onchange.emit(value);
  });

  html! {
      <div class="space-y-0.5">
          if !props.label.is_empty() {
              <label class="font-medium">
                  {props.label.clone()}
              </label>
          }
          <input
              class="px-3 py-2 border focus:outline-blue-500 w-full"
              type="text"
              placeholder={props.placeholder.clone()}
              name={props.name.clone()}
              onchange={onchange}
          />
      </div>
  }
}
