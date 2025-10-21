import { BaseComponent } from "../core/BaseComponent.js";
import { LoudnessMeter } from "./LoudnessMeter.js";
import { VerticalFader } from "./VerticalFader.js";

export class MasterTrack extends BaseComponent {
  constructor(x, y, width, height, options = {}) {
    super();
    const {
      label = "MASTER",
      padding = 12,
      gap = 16,
      background = "#1a1a1a",
      border = "#2a2a2a",
      borderRadius = 8,
      labelOffset = 26,
    } = options;

    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;
    this.isDynamic = true;
    this.label = label;
    this.padding = padding;
    this.gap = gap;
    this.background = background;
    this.border = border;
    this.borderRadius = borderRadius;
    this.labelOffset = labelOffset;

    const innerWidth = width - padding * 2;
    const innerHeight = height - padding * 2;

    const controlsHeight = innerHeight * 0.82;
    const controlsY = y + padding;

    const meterWidth = (innerWidth - gap) / 2;
    const faderWidth = meterWidth;
    const meterX = x + padding;
    const faderX = meterX + meterWidth + gap;

    this.meter = new LoudnessMeter(
      meterX,
      controlsY,
      meterWidth,
      controlsHeight,
      {
        borderRadius: borderRadius * 0.6,
      },
    );

    this.fader = new VerticalFader(
      faderX,
      controlsY,
      faderWidth,
      controlsHeight,
      {
        label: "",
        initialValue: 0.75,
        colors: {
          track: "#252525",
          fill: "#8b9eaa",
          thumb: "#cccccc",
          border: "#1b1b1b",
        },
        borderRadius: borderRadius * 0.6,
      },
    );

    this.meter.setValue(0.5);
    this.fader.setCallback((v) => {
      if (this.onFaderChange) this.onFaderChange(v);
    });
  }

  setMeterValue(v) {
    this.meter.setValue(v);
  }

  update(dt) {
    this.meter.update(dt);
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
    ctx.fillStyle = this.background;
    ctx.fill();
    ctx.strokeStyle = this.border;
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.restore();

    this.meter.draw(ctx);
    this.fader.draw(ctx);

    ctx.save();
    ctx.fillStyle = "#bbb";
    ctx.font = "bold 13px monospace";
    ctx.textAlign = "center";
    ctx.textBaseline = "top";
    const labelX = this.x + this.width / 2;
    const labelY = this.y + this.height - this.labelOffset;
    ctx.fillText(this.label, labelX, labelY);
    ctx.restore();

    this.clearUpdateFlag();
  }

  drawOverlay(ctx) {
    this.fader.drawOverlay(ctx);
  }

  onMouseDown(e) {
    const hit = this.fader.onMouseDown(e) || this.meter.onMouseDown(e);
    if (hit) this.forceUpdate();
    return hit;
  }

  onMouseMove(e) {
    this.fader.onMouseMove(e);
  }

  onMouseUp(e) {
    this.fader.onMouseUp(e);
  }

  onMouseLeave(e) {
    this.fader.onMouseLeave(e);
  }
}
