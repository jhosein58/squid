export class Layout {
  constructor(id, components = []) {
    this.id = id;
    this.components = [];
    this.addComponents(components);
  }

  addComponents(componentsToAdd) {
    const toAdd = Array.isArray(componentsToAdd)
      ? componentsToAdd
      : [componentsToAdd];
    this.components.push(...toAdd);
  }

  update(deltaTime) {
    for (const component of this.components) {
      if (component.isDynamic && component.update) {
        component.update(deltaTime);
      }
    }
  }

  draw(ctx) {
    for (const component of this.components) {
      if (component.draw) {
        component.draw(ctx);
      }
    }
    for (const component of this.components) {
      if (component.drawOverlay) component.drawOverlay(ctx);
    }
  }

  clearUpdateFlags() {
    for (const component of this.components) {
      if (component.clearUpdateFlag) {
        component.clearUpdateFlag();
      }
    }
  }

  needsUpdate() {
    return this.components.some((component) => component.needsUpdate());
  }

  handleMouseDown(event) {
    for (const component of this.components) {
      if (component.onMouseDown) {
        component.onMouseDown(event);
      }
    }
  }

  handleMouseMove(event) {
    for (const component of this.components) {
      if (component.onMouseMove) {
        component.onMouseMove(event);
      }
    }
  }

  handleMouseUp(event) {
    for (const component of this.components) {
      if (component.onMouseUp) {
        component.onMouseUp(event);
      }
    }
  }

  handleKeyDown(event) {
    for (const component of this.components) {
      if (component.onKeyDown) {
        component.onKeyDown(event);
      }
    }
  }

  handleKeyUp(event) {
    for (const component of this.components) {
      if (component.onKeyUp) {
        component.onKeyUp(event);
      }
    }
  }
}
