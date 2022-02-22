use yew::prelude::*;
use crate::components::button::Button;

pub enum Msg{
    Event
}

pub struct Storybook {
    event_clicked: i64
}

impl Component for Storybook {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { event_clicked: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Event => {
                self.event_clicked += 1;
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Default" }</h2>
                <Button text={ "default" } />

                <h2>{ "Event" }</h2>
                <Button text={ "event" } onclick={_ctx.link().callback(|_| Msg::Event)} />
                <div>{ "event clicked " }{ self.event_clicked }{ " times" }</div>
            </>
        }
    }
}
