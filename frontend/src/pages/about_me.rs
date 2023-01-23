use yew::prelude::*;



// Right now the issue is that the elements aren't centered soooo oops.
#[function_component(AboutMe)]
pub fn about_me() -> Html {
	html! {
		<div>
			<br/>
			<div class="flex flex-row-reverse justify-evenly flex-wrap items-center align-items mx-auto p-4">
				<div>
					<h1 class="title text-maroon">{"About me"}</h1>
					// Should increase the size of the text 
					<p class="text-subtext1">{"I am a 20 year old Computer Science Major at the University of Denver. Grew up in Saint Peters, Missouri. Kind of a midwest kid at heart."}</p>
				</div>
			</div>
		</div>
	}
}