import * as wasm from "../gen/toybox_wasm";

var canvas = document.getElementById("breakout");
var ctx = canvas.getContext("2d");

var breakout_sim = wasm.simulator_alloc("breakout");
var state = wasm.state_alloc(breakout_sim);

var frameData = null;;

var KEY_STATE = {};
var alt = 0
function step() {
  if (input == null) {
    frameData = new Uint8ClampedArray(240*160*4);
  }
  
  //alt++;
  if (alt % 2 == 0) {
    ctx.fillStyle= '#f0f';
    ctx.fillRect(0,0,240,160);
    // RGBA8888 data:
    wasm.render_current_frame(frameData, frameData.byteLength, false, breakout_sim, state);

    var input = wasm.WebInput.new();
    input.set_left(KEY_STATE[37])
    input.set_up(KEY_STATE[38])
    input.set_right(KEY_STATE[39])
    input.set_down(KEY_STATE[40])
    input.set_button1(KEY_STATE[32])
    wasm.state_apply_action(state, input);

    var imgData = new ImageData(frameData, 240, 160);
    ctx.putImageData(imgData, 0, 0);
  }
  window.requestAnimationFrame(step);

}

window.addEventListener('keydown',function(evt) {
  KEY_STATE[evt.keyCode] = true;
},true);
window.addEventListener('keyup',function(evt) {
  KEY_STATE[evt.keyCode] = false;
},true);

step();

