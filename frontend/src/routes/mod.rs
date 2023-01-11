use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about_page::Portfilo;
use crate::pages::home_page::Home;
use crate::pages::play_info::Play;

// Routes for page
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/about")]
	Portfilo,
	#[at("/playground")]
	PlaygroundRoot,
	#[at("/playground/*")]
	Playground,
	#[not_found]
	#[at("/404")]
	NotFound,
}

// Playground Sub Route
#[derive(Clone, Routable, PartialEq)]
pub enum PlaygroundRoute {
	#[at("/playground")]
	PlayInfo,
	#[at("/playground/test1")]
	PlayTest1,
	#[not_found]
	#[at("/404")]
	NotFound,
}

// Switches to handle the main routing of the site
pub fn switch(route: Route) -> Html {
	match route {
		Route::Home => html! {<Home />},
        Route::Portfilo => html! {<Portfilo />},
		Route::PlaygroundRoot | Route::Playground => html! { <Switch<PlaygroundRoute> render={switch_playground} /> },
        Route::NotFound => html! {<h1>{"Not Found"}</h1>},
	}
}

// Switch to handle the nested routing for the playground routes
fn switch_playground(route: PlaygroundRoute) -> Html {
	match route {
		PlaygroundRoute::PlayInfo => html! {<Play />},
		PlaygroundRoute::PlayTest1 => html! {<h1>{"PlayTest1"}</h1>},
		PlaygroundRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
	}
}