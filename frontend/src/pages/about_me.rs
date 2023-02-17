use yew::prelude::*;


// Right now the issue is that the elements aren't centered soooo oops.
#[function_component(AboutMe)]
pub fn about_me() -> Html {
	// let card = html1 {
	// 	<Card>
	// 		<CardBody>
	// 			<CardTitle>{"About ME"}</CardTitle>
	// 			<p>{"testing"}</p>
	// 		</CardBody>
	// 	</Card>
	// };

	html! {
		<div>
			<br/>
			<div class="flex">
				// TODO This is where the symbols will go.... Either Experience or Svgs with bullet points




			</div>
			<div class="flex items-center align-items mx-auto p-4 pt-6">
				<div>
					// TODO this is the next section to be working on, cause from here the rest of the site should flow moderately well. Maybe a bit better than the other pages... 
					// I guess it will just have to depend on what happens
					<h1 class="title text-maroon">{"About me"}</h1>
					// Should increase the size of the text 
					<p class="text-subtext1">{"I am a 20 year old Computer Science Major at the University of Denver. Grew up in Saint Peters, Missouri. Kind of a midwest kid at heart."}</p>
				</div>
			</div>
			
		</div>
	}
}