export class LoudnessMeter {
  constructor(x, y, width, height) {
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.value = 0.0;

    this.peakValue = 0.0;
    this.peakHoldTime = 1500;
    this.lastPeakTime = 0;

    this.backgroundColor = "#282828";
    this.gradientStartColor = "#00ff48";
    this.gradientMidColor = "#f6ff00";
    this.gradientEndColor = "#ff0000";
    this.peakColor = "#ffffff";
  }

  update(newValue) {
    this.value = Math.max(0, Math.min(1, newValue));

    if (this.value >= this.peakValue) {
      this.peakValue = this.value;
      this.lastPeakTime = Date.now();
    } else if (Date.now() - this.lastPeakTime > this.peakHoldTime) {
      this.peakValue -= 0.01;
      if (this.peakValue < this.value) {
        this.peakValue = this.value;
      }
    }
  }

  draw(ctx) {
    ctx.fillStyle = this.backgroundColor;
    ctx.fillRect(this.x, this.y, this.width, this.height);

    if (this.value > 0) {
      const meterHeight = this.height * this.value;
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

    if (this.peakValue > 0) {
      const peakHeight = 2;
      const peakY =
        this.y + this.height - this.height * this.peakValue - peakHeight / 2;

      ctx.fillStyle = this.peakColor;
      ctx.fillRect(this.x, peakY, this.width, peakHeight);
    }

    ctx.strokeStyle = "#181818";
    ctx.lineWidth = 1;
    ctx.strokeRect(this.x, this.y, this.width, this.height);
  }
}
