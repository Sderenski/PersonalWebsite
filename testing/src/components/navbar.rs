use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::routes::PlaygroundRoute;

use crate::components::dropdown::Dropdown;
use crate::components::dropdown::DropdownContent;


// To keep track of which link is active, there will have to be a prop change on each link.


#[function_component(PlayGroundMenu)]
fn play_menu() -> Html {
	let navigator = use_navigator().unwrap();

	let _play_info = {
		let navigator = navigator.clone();
		let onclick = Callback::from(move |_| navigator.push(&PlaygroundRoute::PlayInfo));
		html! {
			<li>
				<button class="text-base text-center font-medium text-text hover:text-maroon hover:bg-surface1 
				rounded-lg p-2" aria-current="page" {onclick}>{"Info"}</button>
			</li>
		}
	};

	let _play_test1 = {
		let navigator = navigator.clone();
		let onclick = Callback::from(move |_| navigator.push(&PlaygroundRoute::PlayTest1));
		html! {
			<li>
				<button class="text-base text-center font-medium text-text hover:text-maroon hover:bg-surface1 
				rounded-lg p-2" aria-current="page" {onclick}>{"Test1"}</button>
			</li>
		}
	};

	html! {
		<li>
			<Dropdown>
				<label tabindex="0" class="flex items-center text-center justify-between text-base font-medium text-text hover:text-maroon cursor-pointer">{"Playground"}
					<svg class="w-5 h-5 ml-1" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
						<path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"></path>
					</svg>
				</label>
				<div tabindex="0">
					<DropdownContent class="menu p-2 shadow rounded-lg w-24">
						{_play_info}
						{_play_test1}
					</DropdownContent>
				</div>
			</Dropdown>

		</li>
	}
}


#[function_component(NavBar)]
pub fn nav_items() -> Html {
	let navigator = use_navigator().unwrap();

	let _home_button = {
		let navigator = navigator.clone();
		let onclick = Callback::from(move |_| navigator.push(&Route::Home));
		html! {
			<li>
				<button class="text-base text-center font-medium text-text hover:text-maroon" aria-current="page" {onclick}>{"Home"}</button>
			</li>
		}
	};

	let _portfilo = {
		let navigator = navigator.clone();
		let onclick = Callback::from(move |_| navigator.push(&Route::Portfilo));
		html! {
			<li>
				<button class="text-base text-center font-medium text-text hover:text-maroon" aria-current="page" {onclick}>{"Portfilo"}</button>
			</li>
		}
	};

	let _github = {
		html! {
			<li>
				<a href="https://github.com/Sderenski" class="text-base font-medium">
					<svg width="30px" height="30px" viewBox="0 0 250 250" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMinYMin meet" fill="#000000" stroke="#000000" stroke-width="0">
						<g id="SVGRepo_bgCarrier" stroke-width="0"/>
							<g fill="#ffffff">
							<path d="M127.505 0C57.095 0 0 57.085 0 127.505c0 56.336 36.534 104.13 87.196 120.99 6.372 1.18 8.712-2.766 8.712-6.134 0-3.04-.119-13.085-.173-23.739-35.473 7.713-42.958-15.044-42.958-15.044-5.8-14.738-14.157-18.656-14.157-18.656-11.568-7.914.872-7.752.872-7.752 12.804.9 19.546 13.14 19.546 13.14 11.372 19.493 29.828 13.857 37.104 10.6 1.144-8.242 4.449-13.866 8.095-17.05-28.32-3.225-58.092-14.158-58.092-63.014 0-13.92 4.981-25.295 13.138-34.224-1.324-3.212-5.688-16.18 1.235-33.743 0 0 10.707-3.427 35.073 13.07 10.17-2.826 21.078-4.242 31.914-4.29 10.836.048 21.752 1.464 31.942 4.29 24.337-16.497 35.029-13.07 35.029-13.07 6.94 17.563 2.574 30.531 1.25 33.743 8.175 8.929 13.122 20.303 13.122 34.224 0 48.972-29.828 59.756-58.22 62.912 4.573 3.957 8.648 11.717 8.648 23.612 0 17.06-.148 30.791-.148 34.991 0 3.393 2.295 7.369 8.759 6.117 50.634-16.879 87.122-64.656 87.122-120.973C255.009 57.085 197.922 0 127.505 0"/>
							<path d="M47.755 181.634c-.28.633-1.278.823-2.185.389-.925-.416-1.445-1.28-1.145-1.916.275-.652 1.273-.834 2.196-.396.927.415 1.455 1.287 1.134 1.923M54.027 187.23c-.608.564-1.797.302-2.604-.589-.834-.889-.99-2.077-.373-2.65.627-.563 1.78-.3 2.616.59.834.899.996 2.08.36 2.65M58.33 194.39c-.782.543-2.06.034-2.849-1.1-.781-1.133-.781-2.493.017-3.038.792-.545 2.05-.055 2.85 1.07.78 1.153.78 2.513-.019 3.069M65.606 202.683c-.699.77-2.187.564-3.277-.488-1.114-1.028-1.425-2.487-.724-3.258.707-.772 2.204-.555 3.302.488 1.107 1.026 1.445 2.496.7 3.258M75.01 205.483c-.307.998-1.741 1.452-3.185 1.028-1.442-.437-2.386-1.607-2.095-2.616.3-1.005 1.74-1.478 3.195-1.024 1.44.435 2.386 1.596 2.086 2.612M85.714 206.67c.036 1.052-1.189 1.924-2.705 1.943-1.525.033-2.758-.818-2.774-1.852 0-1.062 1.197-1.926 2.721-1.951 1.516-.03 2.758.815 2.758 1.86M96.228 206.267c.182 1.026-.872 2.08-2.377 2.36-1.48.27-2.85-.363-3.039-1.38-.184-1.052.89-2.105 2.367-2.378 1.508-.262 2.857.355 3.049 1.398"/>
						</g>
					</svg>
				</a>
			</li>
		}
	};

	let _linkedin = {
		html! {
			<li>
				<a href="https://www.linkedin.com/in/stephen-derenski/" class="text-base font-medium">
					<svg width="30px" height="30px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="#000000" stroke="#000000" stroke-width="0">
						<g id="SVGRepo_bgCarrier" stroke-width="0"/> <path fill-rule="evenodd" clip-rule="evenodd" d="M3 1a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2V3a2 2 0 00-2-2H3zm1.102 4.297a1.195 1.195 0 100-2.39 1.195 1.195 0 000 2.39zm1 7.516V6.234h-2v6.579h2zM6.43 6.234h2v.881c.295-.462.943-1.084 2.148-1.084 1.438 0 2.219.953 2.219 2.766 0 .087.008.484.008.484v3.531h-2v-3.53c0-.485-.102-1.438-1.18-1.438-1.079 0-1.17 1.198-1.195 1.982v2.986h-2V6.234z" fill="#ffffff"/> 
					</svg>			
				</a>
			</li>
		}
	};

	

	html! {
		// TODO Change the logo away from the flowbite one to make it my own.
		<header>
			<nav class="border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900">
				<div class="container flex flex-wrap items-center justify-between mx-auto">

					// Change this to its own navigator to the home page as well.
					<ul class="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg  md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0">
						{_home_button}		
						{_portfilo}
											
						// Drop down menu for the about pages
						<PlayGroundMenu />
					</ul>
					<button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false">
						<span class="sr-only">{"Open main menu"}</span>
						<svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
					</button>
					<div class="hidden w-full md:block md:w-auto" id="navbar-default">
						<ul class="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg  md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0">
							{_github}
							{_linkedin}
						</ul>
					</div>
				</div>
			</nav>
		</header>
	}

}


