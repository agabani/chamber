use crate::components::text_field::{TextField, TextFieldEvent};
use yew::prelude::*;

pub enum Msg {
    InputMessage(String),
}

pub struct Storybook {
    event: String,
}

impl Component for Storybook {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            event: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputMessage(value) => {
                self.event = value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Default"} </h2>
                <TextField />

                <h2>{ "Event"} </h2>
                <TextField event={
                    ctx.link().callback(|event| {
                        match event {
                            TextFieldEvent::OnChange(value) => Msg::InputMessage(value),
                        }
                    })
                }/>
                <div>{ "user typed " }{ &self.event }</div>
            </>
        }
    }
}
