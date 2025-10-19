import { AppController } from "./core/AppController.js";
import { Layout } from "./core/Layout.js";
import { Oscilloscope } from "./components/Oscilloscope.js";

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

  const scope = new Oscilloscope(50, 10, 200, 100);

  const mixerLayout = new Layout("mixer", [scope]);
  app.addLayout(mixerLayout);
  const workspaceIds = ["mixer"];

  let phase = 0;
  setInterval(() => {
    const waveform = new Float32Array(256);
    for (let i = 0; i < waveform.length; i++) {
      waveform[i] = Math.sin(phase + (i / waveform.length) * 2 * Math.PI * 3);
    }
    phase += 0.1;

    scope.setValue(waveform);
  }, 50);

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
