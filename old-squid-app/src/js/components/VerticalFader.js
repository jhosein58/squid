import { BaseComponent } from "../core/BaseComponent.js";

export class VerticalFader extends BaseComponent {
  constructor(x, y, width, height, options = {}) {
    super();
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    const {
      onValueChange = () => {},
      initialValue = 0.0,
      label = "",
      colors = {},
      tooltip = {},
      borderRadius = 6,
    } = options;

    this.onValueChange = onValueChange;
    this._externalCallback = null;
    this.label = label;
    this._value = Math.max(0, Math.min(1, initialValue));
    this._isDragging = false;
    this._mousePos = null;

    this.colors = {
      track: colors.track || "#2c2c2c",
      fill: colors.fill || "#6c6c6c",
      thumb: colors.thumb || "#fafafa",
      border: colors.border || "#181818",
      label: colors.label || "#dfdfdf",
    };

    this.borderRadius = borderRadius;

    this.tooltipStyle = {
      background: tooltip.background || "rgba(20, 20, 20, 0.9)",
      color: tooltip.color || "#ffffff",
      font: tooltip.font || "11px monospace",
      paddingX: tooltip.paddingX || 6,
      paddingY: tooltip.paddingY || 3,
      borderRadius: tooltip.borderRadius || 6,
      border: tooltip.border || "1px solid rgba(255,255,255,0.2)",
      offsetX: tooltip.offsetX || 12,
      offsetY: tooltip.offsetY || -16,
    };

    this.labelFont = "12px sans-serif";
  }

  setCallback(fn) {
    this._externalCallback = fn;
  }

  get value() {
    return this._value;
  }

  set value(newValue) {
    const clampedValue = Math.max(0, Math.min(1, newValue));
    if (this._value.toFixed(4) !== clampedValue.toFixed(4)) {
      this._value = clampedValue;
      this.forceUpdate();
      this.onValueChange(this._value);
      if (this._externalCallback) {
        this._externalCallback(this._value);
      }
    }
  }

  _isMouseOver(mouseX, mouseY) {
    return (
      mouseX >= this.x &&
      mouseX <= this.x + this.width &&
      mouseY >= this.y &&
      mouseY <= this.y + this.height
    );
  }

  onMouseDown(event) {
    const rect = event.target.getBoundingClientRect();
    const mouseX = event.clientX - rect.left;
    const mouseY = event.clientY - rect.top;
    if (this._isMouseOver(mouseX, mouseY)) {
      this._isDragging = true;
      this.forceUpdate();
      this.onMouseMove(event);
      return true;
    }
    return false;
  }

  onMouseMove(event) {
    if (!this._isDragging) return;
    const rect = event.target.getBoundingClientRect();
    const mouseY = event.clientY - rect.top;
    const mouseX = event.clientX - rect.left;
    this._mousePos = { x: mouseX, y: mouseY };
    const relativeY = mouseY - this.y;
    this.value = 1.0 - relativeY / this.height;
  }

  onMouseUp() {
    this._isDragging = false;
    this._mousePos = null;
    this.forceUpdate();
  }

  onMouseLeave() {
    this._isDragging = false;
    this._mousePos = null;
    this.forceUpdate();
  }

  drawRoundedRect(ctx, x, y, width, height, r) {
    ctx.beginPath();
    ctx.moveTo(x + r, y);
    ctx.lineTo(x + width - r, y);
    ctx.quadraticCurveTo(x + width, y, x + width, y + r);
    ctx.lineTo(x + width, y + height - r);
    ctx.quadraticCurveTo(x + width, y + height, x + width - r, y + height);
    ctx.lineTo(x + r, y + height);
    ctx.quadraticCurveTo(x, y + height, x, y + height - r);
    ctx.lineTo(x, y + r);
    ctx.quadraticCurveTo(x, y, x + r, y);
    ctx.closePath();
  }

  draw(ctx) {
    ctx.save();
    this.drawRoundedRect(
      ctx,
      this.x,
      this.y,
      this.width,
      this.height,
      this.borderRadius,
    );
    ctx.clip();

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

    ctx.restore();
    ctx.save();

    ctx.strokeStyle = this.colors.border;
    ctx.lineWidth = 1;
    this.drawRoundedRect(
      ctx,
      this.x,
      this.y,
      this.width,
      this.height,
      this.borderRadius,
    );
    ctx.stroke();

    if (this.label) {
      ctx.fillStyle = this.colors.label;
      ctx.font = this.labelFont;
      ctx.textAlign = "center";
      ctx.textBaseline = "top";
      ctx.fillText(
        this.label,
        this.x + this.width / 2,
        this.y + this.height + 5,
      );
    }

    ctx.restore();
    this.clearUpdateFlag();
  }

  drawOverlay(ctx) {
    if (!this._isDragging || !this._mousePos) return;

    const { x, y } = this._mousePos;
    const text = this._value.toFixed(2);
    const {
      background,
      color,
      font,
      paddingX,
      paddingY,
      borderRadius,
      border,
      offsetX,
      offsetY,
    } = this.tooltipStyle;

    ctx.save();
    ctx.font = font;
    const textMetrics = ctx.measureText(text);
    const boxWidth = textMetrics.width + paddingX * 2;
    const boxHeight = (textMetrics.fontBoundingBoxAscent || 12) + paddingY * 2;
    const boxX = x + offsetX;
    const boxY = y + offsetY - boxHeight / 2;

    this.drawRoundedRect(ctx, boxX, boxY, boxWidth, boxHeight, borderRadius);

    ctx.fillStyle = background;
    ctx.fill();

    if (border) {
      const borderParts = border.split(" ");
      ctx.strokeStyle = borderParts.length > 1 ? borderParts[1] : "#fff";
      ctx.lineWidth = parseFloat(borderParts[0]);
      ctx.stroke();
    }

    ctx.fillStyle = color;
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillText(text, boxX + boxWidth / 2, boxY + boxHeight / 2);

    ctx.restore();
  }
}
