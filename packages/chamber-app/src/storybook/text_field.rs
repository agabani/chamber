use crate::components::text_field::TextField;
use yew::prelude::*;

pub enum Msg {
    Onchange(String),
    Oninput(String),
}

pub struct Storybook {
    onchange: String,
    oninput: String,
}

impl Component for Storybook {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            onchange: String::new(),
            oninput: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Onchange(value) => {
                self.onchange = value;
                true
            }
            Msg::Oninput(value) => {
                self.oninput = value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <h2>{ "Default"} </h2>
                <TextField />

                <h2>{ "On Change"} </h2>
                <TextField onchange={link.callback(Msg::Onchange)}/>
                <div>{ "user typed " }{ &self.onchange }</div>

                <h2>{ "On Input"} </h2>
                <TextField oninput={link.callback(Msg::Oninput)}/>
                <div>{ "user typed " }{ &self.oninput }</div>
            </>
        }
    }
}
