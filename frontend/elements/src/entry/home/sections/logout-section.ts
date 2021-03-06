import { MEDIA_UI } from '@utils/path';
 import { LitElement, html, css, customElement, property } from 'lit-element';
 import "@elements/core/titles/variants/title-section";
 import "@elements/core/buttons/rectangle" ;
 import "@elements/entry/home/TOSORT/column-list";

  @customElement('logout-section')
 export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
    }
   main{
    width: 576px;
    height: 352px;
    position:relative;

  }
    
  
  .title{
 left:40px;
    
    position:relative;
    top:86px;
  }
  
  .line{
    margin-left:40px;
    margin-top:7px;
    display:block;
  }
  .button{
    right:40px;
    bottom:40px;
    // display:block;
    position:absolute;

  }

  .textbutton{
    left:40px;
    bottom:50px;
     position:absolute;

  }

   img-ui{
    width: 193px;
    height: 118px;
    position:absolute;
    right:104px;

   }

  .lines{
    margin-top:100px;

  }
 

    `];
  }
  render() {
    const {} = this;


    const STR_TITLE="Logout";
     const STR_LINE1="If you are using a public computer, remember to";
     const STR_LINE2=" log out when you’re done.";
     const STR_LINE3="Do you want to logout? ";
    const STR_LABEL="Logout";
    const STR_Cancel="Cancel";

    return html`
    <main>
    <img-ui path="yellow_Illustration.png"></img-ui>
    <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" class="title"></title-section>

         <div class="lines">

        <column-list class="line" text_line="${STR_LINE1}" size="medium"></column-list>
     <column-list class="line" text_line="${STR_LINE2}" size="medium"></column-list>
     <column-list class="line" text_line="${STR_LINE3}" size="medium"></column-list>

        </div>
         <div class="inside-wrapper">
         <button-rect size="large" color="blue" class="button">${STR_LABEL}</button-rect>
         <button-text  size="large"  color="blue" class="textbutton" >${STR_Cancel}</button-text>
     

         </div>

    </main>
  `;
  }
 }