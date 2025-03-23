import './style.css'
import init, { run } from "./core/pkg/core.js";

async function setup(): Promise<HTMLCanvasElement> {
  await init();
  const app = <HTMLCanvasElement>document.querySelector<HTMLCanvasElement>('#app');
  app.width = app.clientWidth;
  app.height = app.clientHeight;
  return app;
}

async function render(app: HTMLCanvasElement) {
  await run(app);
}

window.onload = async () => {
  const app = await setup();
  await render(app);
};