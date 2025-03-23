import './style.css'
import init, { App } from "./core/generated/core.js";

async function setup(): Promise<App> {
  const canvas = <HTMLCanvasElement>document.querySelector<HTMLCanvasElement>('#app');
  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;
  return App.setup(canvas);
}

function update(app: App, time: number) {
  app.update(time);
}

function render(app: App) {
  app.render();
}


window.onload = async () => {
  await init();
  const app = await setup();

  function gameLoop(time: number) {
    update(app, time / 1000.0) // update time
    render(app);
    requestAnimationFrame(gameLoop);
  }
  requestAnimationFrame(gameLoop);
};