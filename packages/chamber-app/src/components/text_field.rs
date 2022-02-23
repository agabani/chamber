use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    OnChange(String),
}

pub struct TextField;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub event: Option<Callback<TextFieldEvent>>,
}

pub enum TextFieldEvent {
    OnChange(String),
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnChange(value) => {
                if let Some(event) = &ctx.props().event {
                    event.emit(TextFieldEvent::OnChange(value))
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange = link.batch_callback(|event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::OnChange(input.value()))
        });

        html! {
            <input type="text" onchange={onchange} />
        }
    }
}
