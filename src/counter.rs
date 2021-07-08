use yew::prelude::*;

pub struct Counter {
  link: ComponentLink<Self>,
  value: i64,
}

pub enum Msg {
  Increment,
  Decrement
}

impl Counter {
  fn lolcats(&self) -> Html {
    html! {
      <h1>{"Lolcats"}</h1>
    }
  }
}

impl Component for Counter {
  type Message = Msg;
  type Properties = ();
  
  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      value: 0,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Increment => self.value += 1,
      Msg::Decrement => self.value -= 1
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <button onclick=self.link.callback(|_| Msg::Increment)>{"+"}</button>
        <span style="width: 50px">{ self.value }</span>
        <button onclick=self.link.callback(|_| Msg::Decrement)>{"-"}</button>
        {self.lolcats()}
      </div>
    }
}  
}