use yew::prelude::*;

pub struct Text {
  link: ComponentLink<Self>,
  content: String,
}

pub enum Msg {
  Update(String),
}

impl Component for Text {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link, content: "".to_string() }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Update(content) => self.content = content.to_uppercase(),
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <textarea
        oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
        value=&self.content>
      </textarea>
    }
  }
}