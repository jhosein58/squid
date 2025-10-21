import { BaseComponent } from "../core/BaseComponent.js";

export class LoudnessMeter extends BaseComponent {
  constructor(x, y, width, height, options = {}) {
    super();

    const {
      borderRadius = 6,
      backgroundColor = "#282828",
      gradientStartColor = "#00ff48",
      gradientMidColor = "#f6ff00",
      gradientEndColor = "#ff0000",
      peakColor = "#ffffff",
    } = options;

    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.isDynamic = true;

    this._value = 0.0;
    this._peakValue = 0.0;
    this.peakHoldTime = 1500;
    this.lastPeakTime = 0;

    this.borderRadius = borderRadius;
    this.backgroundColor = backgroundColor;
    this.gradientStartColor = gradientStartColor;
    this.gradientMidColor = gradientMidColor;
    this.gradientEndColor = gradientEndColor;
    this.peakColor = peakColor;
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

  setValue(newValue) {
    const clamped = Math.max(0, Math.min(1, newValue));
    if (this._value !== clamped) {
      this._value = clamped;
      this.forceUpdate();
    }
  }

  getValue() {
    return this._value;
  }

  update(deltaTime) {
    const now = Date.now();
    if (this._value >= this._peakValue) {
      this._peakValue = this._value;
      this.lastPeakTime = now;
      this.forceUpdate();
    } else if (now - this.lastPeakTime > this.peakHoldTime) {
      if (this._peakValue > 0) {
        const decay = 0.005 * (deltaTime / 16.67);
        this._peakValue -= decay;
        if (this._peakValue < this._value) this._peakValue = this._value;
        this.forceUpdate();
      }
    }
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

    ctx.fillStyle = this.backgroundColor;
    ctx.fillRect(this.x, this.y, this.width, this.height);

    if (this._value > 0) {
      const meterHeight = this.height * this._value;
      const meterY = this.y + this.height - meterHeight;
      const grad = ctx.createLinearGradient(
        this.x,
        this.y + this.height,
        this.x,
        this.y,
      );
      grad.addColorStop(0, this.gradientStartColor);
      grad.addColorStop(0.7, this.gradientMidColor);
      grad.addColorStop(0.9, this.gradientEndColor);
      ctx.fillStyle = grad;
      ctx.fillRect(this.x, meterY, this.width, meterHeight);
    }

    if (this._peakValue > 0) {
      const peakH = 2;
      const peakY =
        this.y + this.height - this.height * this._peakValue - peakH / 2;
      ctx.fillStyle = this.peakColor;
      ctx.fillRect(this.x, peakY, this.width, peakH);
    }

    ctx.restore();

    ctx.save();
    ctx.strokeStyle = "#181818";
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
    ctx.restore();

    this.clearUpdateFlag();
  }
}
