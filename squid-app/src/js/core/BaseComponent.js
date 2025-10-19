export class BaseComponent {
  _needsUpdate = true;
  isDynamic = false;

  needsUpdate() {
    return this._needsUpdate;
  }
  forceUpdate() {
    this._needsUpdate = true;
  }
  clearUpdateFlag() {
    this._needsUpdate = false;
  }
  constructor() {
    if (new.target === BaseComponent) {
      throw new TypeError(
        "Cannot construct BaseComponent instances directly. You must extend it.",
      );
    }
  }
  draw(ctx) {
    throw new Error(
      `Method "draw(ctx)" must be implemented in ${this.constructor.name}.`,
    );
  }
  update(deltaTime) {}
  onMouseDown(event) {
    return false;
  }
  onMouseMove(event) {}
  onMouseUp(event) {}
  onMouseLeave(event) {}
  onKeyDown(event) {}
  onKeyUp(event) {}
}
