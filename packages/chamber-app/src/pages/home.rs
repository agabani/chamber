use crate::{api::v1::add_one, components::button::Button};
use yew::prelude::*;

pub enum Msg {
    SetValue(i64),
    AddOne,
}

pub struct HomePage {
    value: i64,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                let number = self.value;
                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let result = add_one(number).await;
                    link.send_message(Msg::SetValue(result.data));
                });

                false
            }
            Msg::SetValue(i) => {
                self.value = i;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Button text={ "+1" } onclick={ctx.link().callback(|_| Msg::AddOne)} />
                <p>{ self.value }</p>
            </div>
        }
    }
}
