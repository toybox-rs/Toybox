import * as wasm from "breakout-web-play";

var canvas = document.getElementById("breakout");
var ctx = canvas.getContext("2d");

wasm.greet();

function step() {
  console.log("step");
  window.requestAnimationFrame(step);
}

step();

