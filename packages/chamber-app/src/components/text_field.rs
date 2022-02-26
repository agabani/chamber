use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct TextField {
    initial_value: Option<String>,
}

pub enum Msg {
    OnChange(String),
    OnInput(String),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub value: Option<String>,
    pub onchange: Option<Callback<String>>,
    pub oninput: Option<Callback<String>>,
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let initial_value = props.value.clone();

        Self { initial_value }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.initial_value != props.value
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnChange(value) => {
                let props = ctx.props();
                if let Some(callback) = props.onchange.as_ref() {
                    callback.emit(value)
                }
                false
            }
            Msg::OnInput(value) => {
                let props = ctx.props();
                if let Some(callback) = props.oninput.as_ref() {
                    callback.emit(value)
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();

        let onchange = props.onchange.as_ref().map(|_| {
            link.callback(|event: Event| {
                let input: HtmlInputElement = event.target_unchecked_into::<HtmlInputElement>();
                Msg::OnChange(input.value())
            })
        });

        let oninput = props.oninput.as_ref().map(|_| {
            link.callback(|event: InputEvent| {
                let input: HtmlInputElement = event.target_unchecked_into::<HtmlInputElement>();
                Msg::OnInput(input.value())
            })
        });

        html! {
            <input
                type="text"
                class={classes!("chamber--text-field")}
                value={self.initial_value.clone()}
                onchange={onchange}
                oninput={oninput}
            />
        }
    }
}
