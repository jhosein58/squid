import { LoudnessMeter } from "./components/LoudnessMeter.js";

document.addEventListener("DOMContentLoaded", () => {
  const uiState = {
    activeWorkspace: 0,
    isPlaying: false,
    needsRedraw: true,
  };

  const playBtn = document.getElementById("play-btn");
  const workspacePills = document.querySelectorAll(".workspace-pill");
  const canvas = document.getElementById("main-canvas");
  const ctx = canvas.getContext("2d");

  function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    uiState.needsRedraw = true;
  }

  function renderLoop() {
    if (uiState.needsRedraw) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      if (uiState.activeWorkspace === 1) {
        for (let i = 0; i < 50; i++) {
          const meter = new LoudnessMeter(50 + i * 20, 50, 15, 200);
          meter.update(Math.random());
          meter.draw(ctx);
        }
      }

      uiState.needsRedraw = false;
    }
    requestAnimationFrame(renderLoop);
  }

  function togglePlay() {
    uiState.isPlaying = !uiState.isPlaying;
    playBtn.classList.toggle("playing", uiState.isPlaying);
  }

  function switchWorkspace(workspaceIndex) {
    if (uiState.activeWorkspace === workspaceIndex) return;

    uiState.activeWorkspace = workspaceIndex;
    workspacePills.forEach((pill, index) => {
      pill.classList.toggle("active", index === workspaceIndex);
    });

    uiState.needsRedraw = true;
  }

  playBtn.addEventListener("click", togglePlay);
  workspacePills.forEach((pill, index) => {
    pill.addEventListener("click", () => switchWorkspace(index));
  });
  window.addEventListener("resize", resizeCanvas);

  resizeCanvas();
  switchWorkspace(0);
  renderLoop();
});
