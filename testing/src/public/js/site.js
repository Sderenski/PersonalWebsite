const playBtn = document.getElementById("playBtn");
const playMent = document.getElementById("menuShown");

// Need to create a conditional between the display and hidden for the drop down.
// This all has to stay with the javascript and Tailwindcss functions.

playBtn.addEventListener('click', () => {
	if(playMent.style.display === "none") {
		playMent.style.display = "block";
	} else {
		playMent.style.display = "none";
	}
});
