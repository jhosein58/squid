import { BaseComponent } from "../core/BaseComponent.js";

export class LoudnessMeter extends BaseComponent {
  constructor(x, y, width, height) {
    super();

    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.isDynamic = true;

    this._value = 0.0;
    this._peakValue = 0.0;
    this.peakHoldTime = 1500;
    this.lastPeakTime = 0;

    this.backgroundColor = "#282828";
    this.gradientStartColor = "#00ff48";
    this.gradientMidColor = "#f6ff00";
    this.gradientEndColor = "#ff0000";
    this.peakColor = "#ffffff";
  }

  setValue(newValue) {
    const clampedValue = Math.max(0, Math.min(1, newValue));
    if (this._value !== clampedValue) {
      this._value = clampedValue;
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
        const decayAmount = 0.005 * (deltaTime / 16.67);
        this._peakValue -= decayAmount;

        if (this._peakValue < this._value) {
          this._peakValue = this._value;
        }
        this.forceUpdate();
      }
    }
  }

  draw(ctx) {
    ctx.fillStyle = this.backgroundColor;
    ctx.fillRect(this.x, this.y, this.width, this.height);

    if (this._value > 0) {
      const meterHeight = this.height * this._value;
      const meterY = this.y + this.height - meterHeight;

      const gradient = ctx.createLinearGradient(
        this.x,
        this.y + this.height,
        this.x,
        this.y,
      );
      gradient.addColorStop(0, this.gradientStartColor);
      gradient.addColorStop(0.7, this.gradientMidColor);
      gradient.addColorStop(0.9, this.gradientEndColor);

      ctx.fillStyle = gradient;
      ctx.fillRect(this.x, meterY, this.width, meterHeight);
    }

    if (this._peakValue > 0) {
      const peakIndicatorHeight = 2;
      const peakY =
        this.y +
        this.height -
        this.height * this._peakValue -
        peakIndicatorHeight / 2;

      ctx.fillStyle = this.peakColor;
      ctx.fillRect(this.x, peakY, this.width, peakIndicatorHeight);
    }

    ctx.strokeStyle = "#181818";
    ctx.lineWidth = 1;
    ctx.strokeRect(this.x, this.y, this.width, this.height);
  }
}
