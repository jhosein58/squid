import { AppController } from "./core/AppController.js";
import { Layout } from "./core/Layout.js";
import { Oscilloscope } from "./components/Oscilloscope.js";
import { VerticalFader } from "./components/VerticalFader.js";
import { MasterTrack } from "./components/MasterTrack.js";
import { MixerChannel } from "./components/MixerChannel.js";
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

  const scope = new Oscilloscope(10, 10, 300, 100);

  const mixerLayout = new Layout("mixer", [
    scope,
    new MasterTrack(10, 120, 80, 400),
    new MixerChannel(100, 120, 60, 400),
    new MixerChannel(170, 120, 60, 400),
    new MixerChannel(240, 120, 60, 400),
    new MixerChannel(310, 120, 60, 400),
    new MixerChannel(380, 120, 60, 400),
    new MixerChannel(450, 120, 60, 400),
    new MixerChannel(520, 120, 60, 400),
    new MixerChannel(590, 120, 60, 400),
    new MixerChannel(660, 120, 60, 400),
  ]);
  app.addLayout(mixerLayout);
  const workspaceIds = ["mixer"];

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

  window.__TAURI__.event.listen("oscilloscope_waveform", (event) => {
    scope.setValue(event.payload);
  });
});
