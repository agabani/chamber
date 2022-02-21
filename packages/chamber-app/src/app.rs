use yew::prelude::*;

use crate::router::Router;

pub enum Msg {}

pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Router />
        }
    }
}
