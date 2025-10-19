export class VerticalFader {
  constructor(x, y, width, height, options = {}) {
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    const {
      onValueChange = () => {},
      initialValue = 0.0,
      label = "",
    } = options;
    this.onValueChange = onValueChange;
    this.label = label;

    this._value = Math.max(0, Math.min(1, initialValue));
    this._previousValue = -1;
    this._needsUpdate = true;

    this._isDragging = false;

    this.colors = {
      track: "#282828",
      fill: "#5a5a5a",
      thumb: "#e0e0e0",
      border: "#181818",
      label: "#cccccc",
    };
    this.labelFont = "12px sans-serif";
  }

  get value() {
    return this._value;
  }

  set value(newValue) {
    const clampedValue = Math.max(0, Math.min(1, newValue));
    if (this._value.toFixed(4) !== clampedValue.toFixed(4)) {
      this._value = clampedValue;
      this._needsUpdate = true;
      this.onValueChange(this._value);
    }
  }

  needsUpdate() {
    const valueChanged =
      this._value.toFixed(3) !== this._previousValue.toFixed(3);
    return this._needsUpdate || valueChanged;
  }

  _isMouseOver(mouseX, mouseY) {
    return (
      mouseX >= this.x &&
      mouseX <= this.x + this.width &&
      mouseY >= this.y &&
      mouseY <= this.y + this.height
    );
  }

  handleMouseDown(event) {
    if (this._isMouseOver(event.clientX, event.clientY)) {
      this._isDragging = true;
      this._needsUpdate = true;
      this.handleMouseMove(event);
    }
  }

  handleMouseMove(event) {
    if (this._isDragging) {
      const relativeY = event.clientY - this.y;
      this.value = 1.0 - relativeY / this.height;
    }
  }

  handleMouseUp() {
    this._isDragging = false;
    this._needsUpdate = true;
  }

  draw(ctx) {
    this._previousValue = this._value;

    ctx.fillStyle = this.colors.track;
    ctx.fillRect(this.x, this.y, this.width, this.height);

    const fillHeight = this.height * this._value;
    const fillY = this.y + this.height - fillHeight;
    ctx.fillStyle = this.colors.fill;
    ctx.fillRect(this.x, fillY, this.width, fillHeight);

    const thumbY = fillY;
    ctx.strokeStyle = this.colors.thumb;
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(this.x, thumbY);
    ctx.lineTo(this.x + this.width, thumbY);
    ctx.stroke();

    ctx.strokeStyle = this.colors.border;
    ctx.lineWidth = 1;
    ctx.strokeRect(this.x, this.y, this.width, this.height);

    if (this.label) {
      ctx.fillStyle = this.colors.label;
      ctx.font = this.labelFont;
      ctx.textAlign = "center";
      ctx.fillText(
        this.label,
        this.x + this.width / 2,
        this.y + this.height + 15,
      );
    }

    this._needsUpdate = false;
  }
}
