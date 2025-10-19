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
      if (component.handleMouseDown) {
        component.handleMouseDown(event);
      }
    }
  }

  handleMouseMove(event) {
    for (const component of this.components) {
      if (component.handleMouseMove) {
        component.handleMouseMove(event);
      }
    }
  }

  handleMouseUp(event) {
    for (const component of this.components) {
      if (component.handleMouseUp) {
        component.handleMouseUp(event);
      }
    }
  }

  handleKeyDown(event) {
    for (const component of this.components) {
      if (component.handleKeyDown) {
        component.handleKeyDown(event);
      }
    }
  }

  handleKeyUp(event) {
    for (const component of this.components) {
      if (component.handleKeyUp) {
        component.handleKeyUp(event);
      }
    }
  }
}
