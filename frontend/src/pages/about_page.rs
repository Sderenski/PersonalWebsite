use yew::prelude::*;

use crate::components::navbar::NavBar;

#[function_component(Portfilo)]
pub fn portfilo() -> Html {
	html! {
		<div>
			<NavBar />
			<div>
				<h1 class="text-3xl font-bold underline">{"Profile"}</h1>
			</div>
		</div>
	}
}