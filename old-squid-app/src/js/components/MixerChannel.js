import { BaseComponent } from "../core/BaseComponent.js";
import { LoudnessMeter } from "./LoudnessMeter.js";
import { VerticalFader } from "./VerticalFader.js";

export class MixerChannel extends BaseComponent {
  constructor(x, y, width, height, options = {}) {
    super();
    const {
      label = "CH1",
      padding = 10,
      gap = 6,
      background = "#1a1a1a",
      border = "#2a2a2a",
      borderRadius = 8,
      labelOffset = 26,
      meterHeightRatio = 0.25,
      meterGapBelow = 10,
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

    const metersHeight = innerHeight * meterHeightRatio;
    const metersY = y + padding;

    const meterWidth = (innerWidth - gap) / 2;
    const meterLeftX = x + padding;
    const meterRightX = meterLeftX + meterWidth + gap;

    this.meterL = new LoudnessMeter(
      meterLeftX,
      metersY,
      meterWidth,
      metersHeight,
      {
        borderRadius: borderRadius * 0.4,
      },
    );

    this.meterR = new LoudnessMeter(
      meterRightX,
      metersY,
      meterWidth,
      metersHeight,
      {
        borderRadius: borderRadius * 0.4,
      },
    );

    const faderY = metersY + metersHeight + meterGapBelow;
    const faderHeight = innerHeight - metersHeight - meterGapBelow;
    const faderX = x + padding;
    const faderWidth = innerWidth;

    this.fader = new VerticalFader(faderX, faderY, faderWidth, faderHeight, {
      label: "",
      initialValue: 0.75,
      colors: {
        track: "#252525",
        thumb: "#cccccc",
        border: "#1b1b1b",
      },
      borderRadius: borderRadius * 0.5,
    });

    this.meterL.setValue(Math.random());
    this.meterR.setValue(Math.random());

    this.fader.setCallback((v) => {
      if (this.onFaderChange) this.onFaderChange(v);
    });
  }

  setMeterValues(left, right) {
    this.meterL.setValue(left);
    this.meterR.setValue(right);
  }

  update(dt) {
    this.meterL.update(dt);
    this.meterR.update(dt);
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

    this.meterL.draw(ctx);
    this.meterR.draw(ctx);
    this.fader.draw(ctx);

    ctx.save();
    ctx.fillStyle = "#bbb";
    ctx.font = "bold 12px monospace";
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
    const hit =
      this.fader.onMouseDown(e) ||
      this.meterL.onMouseDown(e) ||
      this.meterR.onMouseDown(e);
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
