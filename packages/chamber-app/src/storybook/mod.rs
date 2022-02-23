use yew::prelude::*;
use yew_router::prelude::*;
mod button;
mod text_field;

#[derive(Clone, PartialEq, Routable)]
pub enum StorybookRoute {
    #[at("/storybook/button")]
    Button,
    #[at("/storybook/text_field")]
    TextField,
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
                            StorybookRoute::TextField => html! { <text_field::Storybook /> },
                        }
                    })
                }/>
            </>
        }
    }
}
