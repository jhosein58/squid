export class AppController {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");

    this.layouts = new Map();
    this.activeLayout = null;
    this.dpr = 1;

    this.isRunning = false;
    this._lastTimestamp = 0;

    this._boundRenderLoop = this.renderLoop.bind(this);
    this._boundHandleResize = this.handleResize.bind(this);

    this._initEventListeners();
  }

  addLayout(layout) {
    if (!layout || !layout.id) {
      console.error("Invalid layout provided. Layout must have an ID.");
      return;
    }
    this.layouts.set(layout.id, layout);
  }

  addLayouts(layouts) {
    if (!Array.isArray(layouts)) {
      layouts = [layouts];
    }
    for (const layout of layouts) {
      this.addLayout(layout);
    }
  }

  switchLayout(layoutId) {
    if (!this.layouts.has(layoutId)) {
      console.error(`Layout "${layoutId}" not found.`);
      return;
    }

    this.activeLayout = this.layouts.get(layoutId);
    console.log(`Switched to layout: ${layoutId}`);

    this.requestFullRedraw();
  }

  _initEventListeners() {
    window.addEventListener("resize", this._boundHandleResize);

    const eventHandler = (handlerName, event) => {
      if (this.activeLayout && this.activeLayout[handlerName]) {
        this.activeLayout[handlerName](event);
      }
    };

    this.canvas.addEventListener("mousedown", (e) =>
      eventHandler("handleMouseDown", e),
    );
    this.canvas.addEventListener("mousemove", (e) =>
      eventHandler("handleMouseMove", e),
    );
    this.canvas.addEventListener("mouseup", (e) =>
      eventHandler("handleMouseUp", e),
    );

    window.addEventListener("keydown", (e) => eventHandler("handleKeyDown", e));
    window.addEventListener("keyup", (e) => eventHandler("handleKeyUp", e));
  }

  handleResize() {
    this.dpr = window.devicePixelRatio || 1;
    const rect = this.canvas.getBoundingClientRect();

    this.canvas.width = rect.width * this.dpr;
    this.canvas.height = rect.height * this.dpr;

    this.ctx.scale(this.dpr, this.dpr);

    console.log(
      `Canvas resized to: ${rect.width}x${rect.height} (DPR: ${this.dpr})`,
    );

    this.requestFullRedraw();
  }

  requestFullRedraw() {
    if (this.activeLayout) {
      this.activeLayout.components.forEach((component) =>
        component.forceUpdate(),
      );
    }
  }

  renderLoop(timestamp) {
    if (!this.isRunning) return;

    if (this._lastTimestamp === 0) {
      this._lastTimestamp = timestamp;
    }
    const deltaTime = timestamp - this._lastTimestamp;

    if (this.activeLayout) {
      this.activeLayout.update(deltaTime);

      if (this.activeLayout.needsUpdate()) {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.activeLayout.draw(this.ctx);
        this.activeLayout.clearUpdateFlags();
      }
    }

    this._lastTimestamp = timestamp;
    requestAnimationFrame(this._boundRenderLoop);
  }

  start() {
    if (this.isRunning) return;
    console.log("AppController started.");
    this.isRunning = true;
    this.handleResize();
    this._lastTimestamp = 0;
    requestAnimationFrame(this._boundRenderLoop);
  }

  stop() {
    if (!this.isRunning) return;
    console.log("AppController stopped.");
    this.isRunning = false;
  }
}
