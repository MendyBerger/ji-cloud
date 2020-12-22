import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('input-text')
export class _ extends LitElement {

  static get styles() {
    return [css`
   .input-wrapper{
    position:relative;
    width:296px;
    height:64px;
    border: solid 1px #89b3ff;
    border-radius:14px;
    padding: 8px 48px 8px 16px;
   }
   .instruction{
     display:none;
     margin-left: 8px;
   }
   input{
    outline:none;
    border:none;
    margin-top: 33px;
    width: 296px;
   }
   label{
    position:absolute;
    top: 0;
    left: 0;
    font-size: 16px;
    padding: 8px 0px 0px 16px;
    color: #5590fc;
   }
   .input-wrapper:active{
    border: solid 2px #5590fc;
   }
   input{ font-size:16px;}
   .helpertext{
     font-size: 14px;
     color: #5590fc;
     margin-top:4px;
     font-weight: 500;
   }
   active .helpertext{
    display:block;
   }
   .error{
    font-size: 14px;
    color: #f00813;
    margin-top:4px;
    font-weight: 500;
    padding-left:8px;
    display:block;
   }
   p{display:none;}
   
  
    `];
  }

  @property()
  label: string = "";

  @property()
  instruction: boolean = false;

  @property()
  helpertext: string = "";

  @property()
  error: boolean = false;

  render() {

    const {label, helpertext, error, instruction} = this;

    const classes = classMap({ 
      instruction,
      error,
    });


    return html`
    
    <div class="input-wrapper">
        <input placeholder="Placeholder" type="text" class="">
        <label class="">${label}</label>
    </div>
    <p class="">${helpertext}</p>
    <p class="">${error}</p>
     
  `;
  }
}