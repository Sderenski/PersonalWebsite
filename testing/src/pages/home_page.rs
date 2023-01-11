use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::components::navbar::NavBar;
use crate::pages::about_me::AboutMe;

use crate::routes::Route;

#[function_component]
pub fn Home() -> Html {
	let navigator = use_navigator().unwrap();

	let _portfilo = {
		let navigator = navigator.clone();
		let onclick = {Callback::from(move |_| navigator.push(&Route::Portfilo))};
		html! {
			<button class="p-2 border-2 rounded-xl text-base font-medium border-maroon text-maroon transition-colors ease-in-out hover:bg-maroon/[.1] duration-650" {onclick}>{"Check out my Portfilo"}</button>
		}
	};

	html! {
		<body class="background-color overscroll-contain">
			<NavBar />
			// Might change this to be more relative, Like over to the left and down more.
				<div class="relative h-screen">
					<div class="absolute pl-20 pr-20 max-w-2xl md:bottom-80 md:left-48 bottom-40 left-16">
					// In here I want something popping for a landing page for a software engineer... Maybe animated but I'll need to learn CSS to another level.
						<div>
							<h3 class="text-maroon">{"Hi, my name is"}</h3>
							<h1 class="title text-subtext1">{"Stephen Derenski"}</h1>
							<h2 class="title text-subtext1 break-after-page">{"I am a student"}</h2>
						</div>
						<br />
						<p class="text-subtext1">{"Currently a sophmore at the University of Denver. Studying Computer Science with an emphasis in Software Engineering and Architecture."}</p>
						<br />
						{_portfilo}
					</div>
					<div class="absolute bottom-32 left-0 right-0 w-14 ml-auto mr-auto">
						<a href="#">
							<svg class="animate-bounce hover:cursor-pointer" fill="#eba0ac" height="50px" width="50px" version="1.1" id="SVGRepoEditor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 407.437 407.437" stroke="#eba0ac" stroke-width="0">
								<g id="SVGRepo_bgCarrier" stroke-width="0"/> 
								<polygon points="386.258,91.567 203.718,273.512 21.179,91.567 0,112.815 203.718,315.87 407.437,112.815 "/> 
							</svg>
						</a>
					</div>
				</div>
				// Whole Ass next section. Fuck me man. This is a lot of design.
				<div class="h-screen">
					// This section will have to require a lot of time to figure out how the flow will be... Yikes
					<AboutMe />
				</div>
		</body>
	}
}
