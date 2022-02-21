use yew::prelude::*;

pub struct Button;

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub onclick: Option<Callback<OnClick>>,
}

pub enum OnClick {
    Clicked,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.props().onclick.as_ref().map(|callback| {
            let callback = callback.clone();
            move |_| callback.emit(OnClick::Clicked)
        });

        html! {
            <button
                class={classes!("chamber--button")}
                onclick={onclick}
            >
                { &ctx.props().text }
            </button>
        }
    }
}
