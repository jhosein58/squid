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

  let f1 = new VerticalFader(100, 120, 60, 400, { label: "Waveform" });
  f1.setCallback((v) => {
    window.__TAURI__.core.invoke("set_f1", {
      f1: v,
      velocity: 100,
    });
  });

  let f2 = new VerticalFader(170, 120, 60, 400, { label: "Delay" });
  f2.setCallback((v) => {
    window.__TAURI__.core.invoke("set_f2", {
      f2: v,
      velocity: 100,
    });
  });
  let f3 = new VerticalFader(240, 120, 60, 400, { label: "Bitcrusher" });
  f3.setCallback((v) => {
    window.__TAURI__.core.invoke("set_f3", {
      f3: v,
      velocity: 100,
    });
  });

  const mixerLayout = new Layout("mixer", [
    scope,
    new MasterTrack(10, 120, 80, 400),
    f1,
    f2,
    f3,
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

  const keyToNote = {
    // ------- Octave 4 -------
    z: 60, // C4
    s: 61, // C#4
    x: 62, // D4
    d: 63, // D#4
    c: 64, // E4
    v: 65, // F4
    g: 66, // F#4
    b: 67, // G4
    h: 68, // G#4
    n: 69, // A4
    j: 70, // A#4
    m: 71, // B4

    // ------- Octave 5 -------
    q: 72, // C5
    2: 73, // C#5
    w: 74, // D5
    3: 75, // D#5
    e: 76, // E5
    r: 77, // F5
    5: 78, // F#5
    t: 79, // G5
    6: 80, // G#5
    y: 81, // A5
    7: 82, // A#5
    u: 83, // B5
    i: 84, // C6
    9: 85, // C#6
    o: 86, // D6
    0: 87, // D#6
    p: 88, // E6
  };

  window.addEventListener("keydown", (e) => {
    const key = e.key.toLowerCase();
    if (keyToNote[key] !== undefined) {
      const note = keyToNote[key];
      window.__TAURI__.core.invoke("push_note_event", {
        note: note,
        velocity: 100,
      });
    }
  });

  window.addEventListener("keyup", (e) => {
    const key = e.key.toLowerCase();

    if (keyToNote[key] !== undefined) {
      const note = keyToNote[key];
      window.__TAURI__.core.invoke("note_off_event", {
        note: note,
      });
    }
  });
});
