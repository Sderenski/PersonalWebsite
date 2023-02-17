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


// This is from a code thingy to animate the title.

// Wrap every letter in a span
var textWrapper = document.querySelector('.ml12');
textWrapper.innerHTML = textWrapper.textContent.replace(/\S/g, "<span class='letter'>$&</span>");

anime.timeline({loop: true})
  .add({
    targets: '.ml12 .letter',
    translateX: [40,0],
    translateZ: 0,
    opacity: [0,1],
    easing: "easeOutExpo",
    duration: 1200,
    delay: (el, i) => 500 + 30 * i
  }).add({
    targets: '.ml12 .letter',
    translateX: [0,-30],
    opacity: [1,0],
    easing: "easeInExpo",
    duration: 1100,
    delay: (el, i) => 100 + 30 * i
  });