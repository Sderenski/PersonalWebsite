use yew::prelude::*;

use crate::components::navbar::NavBar;

#[function_component]
pub fn Play() -> Html {
	html! {
		<div>
			<NavBar />
			<div>
				<h1>{"PlayInfo"}</h1>
			</div>
		</div>
	}
}