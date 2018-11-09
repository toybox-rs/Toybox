import * as wasm from "breakout-web-play";

var canvas = document.getElementById("breakout");
var ctx = canvas.getContext("2d");

var breakout_sim = wasm.simulator_alloc("breakout");
var state = wasm.state_alloc(breakout_sim);

function step() {
  ctx.fillStyle= '#f0f';
  ctx.fillRect(0,0,240,160);
  // RGBA8888 data:
  var frameData = new Uint8ClampedArray(240*160*4);
  wasm.render_current_frame(frameData, frameData.byteLength, false, breakout_sim, state);
  var data = {'0': 1}
  for (var i=0; i<256; i++) {
    data[''+i] = 0
  }
  for (var f=0; f<1000; f++) {
    data[''+frameData[f]]++;
  }
  //console.log(data);
  
  var imgData = new ImageData(frameData, 240, 160);
  ctx.putImageData(imgData, 0, 0);
  window.requestAnimationFrame(step);
}

step();

