use yew::prelude::*;
use yew_router::prelude::*;
mod button;

#[derive(Clone, PartialEq, Routable)]
pub enum StorybookRoute {
    #[at("/storybook/button")]
    Button,
}

pub enum Msg {}

pub struct Storybook;

impl Component for Storybook {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Storybook" }</h1>
                <Switch<StorybookRoute> render={
                    Switch::render(|route| {
                        match route {
                            StorybookRoute::Button => html! { <button::Storybook /> },
                        }
                    })
                }/>
            </>
        }
    }
}
