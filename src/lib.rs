mod counter;
use counter::Counter;

mod text;
use text::Text;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Model {}

impl Component for Model {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <Counter />
        <Counter />
        <Text />
        <Text />
      </div>  
    }
  }
}

#[wasm_bindgen(start)]
pub fn run_app() {
  App::<Model>::new().mount_to_body();
}