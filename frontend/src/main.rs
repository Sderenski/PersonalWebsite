use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod routes;
mod components;
use crate::routes::{Route, switch};

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
