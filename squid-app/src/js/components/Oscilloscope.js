import { BaseComponent } from "../core/BaseComponent.js";

export class Oscilloscope extends BaseComponent {
  _waveformData = [];

  constructor(x, y, width, height, options = {}) {
    super();

    const {
      strokeColor = "#33ff99",
      lineWidth = 2,
      backgroundColor = "rgba(0, 0, 0, 0.5)",
      borderColor = "#222",
      borderWidth = 1,
      borderRadius = 8,
    } = options;

    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.strokeColor = strokeColor;
    this.lineWidth = lineWidth;
    this.backgroundColor = backgroundColor;
    this.borderColor = borderColor;
    this.borderWidth = borderWidth;
    this.borderRadius = borderRadius;
  }

  setValue(data) {
    if (!data || data.length === 0) {
      this._waveformData = [];
    } else {
      this._waveformData = data;
    }
    this.forceUpdate();
  }

  drawRoundedRect(ctx, x, y, w, h, r) {
    ctx.beginPath();
    ctx.moveTo(x + r, y);
    ctx.lineTo(x + w - r, y);
    ctx.quadraticCurveTo(x + w, y, x + w, y + r);
    ctx.lineTo(x + w, y + h - r);
    ctx.quadraticCurveTo(x + w, y + h, x + w - r, y + h);
    ctx.lineTo(x + r, y + h);
    ctx.quadraticCurveTo(x, y + h, x, y + h - r);
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
    ctx.fillStyle = this.backgroundColor;
    ctx.fill();

    ctx.strokeStyle = this.borderColor;
    ctx.lineWidth = this.borderWidth;
    ctx.stroke();

    ctx.clip();

    const data = this._waveformData;
    const dataLength = data.length;

    ctx.strokeStyle = this.strokeColor;
    ctx.lineWidth = this.lineWidth;

    ctx.beginPath();

    if (dataLength < 2) {
      const centerY = this.y + this.height / 2;
      ctx.moveTo(this.x, centerY);
      ctx.lineTo(this.x + this.width, centerY);
      ctx.stroke();
      ctx.restore();
      return;
    }

    const centerY = this.y + this.height / 2;
    const amplitude = (this.height - this.lineWidth) / 2;
    const drawableWidth = this.width - this.lineWidth - this.borderWidth;
    const xOffset = this.x + this.lineWidth / 2 + this.borderWidth / 2;

    ctx.moveTo(xOffset, centerY - data[0] * amplitude);
    for (let i = 1; i < dataLength; i++) {
      const xPos = xOffset + (i / (dataLength - 1)) * drawableWidth;
      const yPos = centerY - data[i] * amplitude;
      ctx.lineTo(xPos, yPos);
    }

    ctx.stroke();

    ctx.restore();
  }
}
