use yew_router::prelude::*;

use crate::pages::home::HomePage;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

use yew::prelude::*;

pub enum Msg {}

pub struct Router;

impl Component for Router {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(|route| {
                    match route {
                        Route::Home => html! { <HomePage /> },
                        Route::NotFound => html! { <h1>{ "404" }</h1> },
                    }
                }
            )} />
            </BrowserRouter>
        }
    }
}
