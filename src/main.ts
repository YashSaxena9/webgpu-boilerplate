import './style.css'
import init, { App } from "./core/generated/core.js";

let mouseX = 0;
let mouseY = 0;

async function setup(): Promise<App> {
  const canvas = <HTMLCanvasElement>document.querySelector<HTMLCanvasElement>('#app');
  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;
  const app = await App.setup(canvas);
  if (app.requested_input.mouse_position) {
    canvas.onmousemove = (event) => {
      mouseX = event.clientX - canvas.offsetLeft;
      mouseY = event.clientY - canvas.offsetTop;
    }
  }
  return app;
}

function update(app: App, time: number, delta: number) {
  app.update(
    app.requested_input.elapsed_time   ? time   : null,
    app.requested_input.delta_time     ? delta  : null,
    app.requested_input.mouse_position ? mouseX : null,
    app.requested_input.mouse_position ? mouseY : null,
  );
}

function render(app: App) {
  app.render();
}


window.onload = async () => {
  await init();
  const app = await setup();
  let prevTime = 0;
  function gameLoop(time: number) {
    const delta = (time - prevTime) / 1000.0;
    update(app, time / 1000.0, delta); // update time
    render(app);
    prevTime = time;
    requestAnimationFrame(gameLoop);
  }
  requestAnimationFrame(gameLoop);
};