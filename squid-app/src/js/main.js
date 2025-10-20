import { AppController } from "./core/AppController.js";
import { Layout } from "./core/Layout.js";
import { Oscilloscope } from "./components/Oscilloscope.js";

const invoke = window.__TAURI__.core.invoke;

document.addEventListener("DOMContentLoaded", () => {
  const canvas = document.getElementById("main-canvas");
  const playBtn = document.getElementById("play-btn");
  const workspacePills = document.querySelectorAll(".workspace-pill");

  if (!canvas) {
    console.error("Canvas element with id 'main-canvas' not found.");
    return;
  }
  canvas.style.background = "#1e1e1e";

  const app = new AppController(canvas);

  const scope = new Oscilloscope(20, 20, 600, 100);

  const mixerLayout = new Layout("mixer", [scope]);
  app.addLayout(mixerLayout);
  const workspaceIds = ["mixer"];

  setInterval(() => {
    invoke("get_frequency")
      .then((res) => scope.setValue(res.splice(0, 600)))
      .catch((err) => console.error("Error:", err));
  }, 20);

  let isPlaying = false;
  function togglePlay() {
    isPlaying = !isPlaying;
    playBtn.classList.toggle("playing", isPlaying);
  }

  function switchWorkspace(workspaceIndex) {
    const layoutId = workspaceIds[workspaceIndex];
    if (layoutId) {
      app.switchLayout(layoutId);

      workspacePills.forEach((pill, index) => {
        pill.classList.toggle("active", index === workspaceIndex);
      });
    } else {
      console.error(`Workspace with index ${workspaceIndex} is not defined.`);
    }
  }

  playBtn.addEventListener("click", togglePlay);
  workspacePills.forEach((pill, index) => {
    pill.addEventListener("click", () => switchWorkspace(index));
  });

  switchWorkspace(0);
  app.start();
});
