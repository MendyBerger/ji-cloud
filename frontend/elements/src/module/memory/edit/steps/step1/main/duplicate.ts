import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_EMPTY = "No preview yet";

@customElement('step1-main-duplicate')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              width: 100%;
              height: 100%;
              display: flex;
              justify-content: center;
              align-items: center;
          }
    `];
  }

  render() {
      return html`
          <section class="empty">
              Cards here..
          </section>
      `
  }
}
