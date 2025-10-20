import { BaseComponent } from "../core/BaseComponent.js";

export class Oscilloscope extends BaseComponent {
  _waveformData = [];

  constructor(x, y, width, height) {
    super();

    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.strokeColor = "#33ff99";
    this.lineWidth = 2;
    this.backgroundColor = "rgba(0, 0, 0, 0.5)";
  }

  setValue(data) {
    if (!data || data.length === 0) {
      this._waveformData = [];
    } else {
      this._waveformData = data;
    }
    this.forceUpdate();
  }

  draw(ctx) {
    ctx.save();

    ctx.beginPath();
    ctx.rect(this.x, this.y, this.width, this.height);

    ctx.fillStyle = this.backgroundColor;
    ctx.fill();
    ctx.clip();

    ctx.strokeStyle = this.strokeColor;
    ctx.lineWidth = this.lineWidth;

    ctx.beginPath();

    const data = this._waveformData;
    const dataLength = data.length;

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
    const drawableWidth = this.width - this.lineWidth;
    const xOffset = this.x + this.lineWidth / 2;

    const firstX = xOffset;
    const firstY = centerY - data[0] * amplitude;
    ctx.moveTo(firstX, firstY);

    for (let i = 1; i < dataLength; i++) {
      const xPos = xOffset + (i / (dataLength - 1)) * drawableWidth;
      const yPos = centerY - data[i] * amplitude;
      ctx.lineTo(xPos, yPos);
    }

    ctx.stroke();

    ctx.restore();
  }
}
