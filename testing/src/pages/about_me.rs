use yew::prelude::*;


#[function_component(AboutMe)]
pub fn about_me() -> Html {
	html! {
		<div class="flex flex-row-reverse justify-evenly flex-wrap items-center mx-auto">
			<div>
				<img class="proImg" src="imgs/IMG-1579.JPG" />
			</div>	
			<div>
				<p>{"Testing"}</p>
			</div>
		</div>
	}
}