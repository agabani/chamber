mod components;

use yew::prelude::*;

use components::button::Button;

pub enum Msg {
    AddOne,
}

pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <Button text={ "+1" } onclick={link.callback(|_| Msg::AddOne)} />
                <p>{ self.value }</p>
            </div>
        }
    }
}
