import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/subtitle";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/home/TOSORT/column-list"
import "@elements/core/buttons/rectangle";

@customElement('create-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
      display:flex;
      background-color:#fffde6; 
      width: 1920px;
      height: 600px;
  
  
     }

    .inside-wrapper{
       display:flex;
       margin-left:70px;
       margin-top: -90px;

    }
  
   
   .gears{
    width: 325px;
    height: 355px;
    bottom:30px;
    right:40px;
    position: absolute;

   }
   
  .subtitle{
     margin-top: 65px;
     margin-left:80px;
     display:block;
  
  }

  .title{
    margin-left:10px;
    display:block;
 
 }

  .line{
    display:flex;
    align-items:center;
    margin-top:16px;
    margin-left:80px;

    }

.Start-creating{
        margin-top:50px;
        display:block;
        margin-left:80px;

          
        }

.right_side{
    width: 853px;
    height: 600px;
    position: relative;
  }
.left_side{
    width: 1067px;
    height: 600px;

}
.line::before{
  content:'';
  height:12px;
  width:12px;
  border-radius:50%;
  background-color: #fed758;
  margin-right:16px;

}



    `];
  }
  @property()
  STR_PATHGEARS:string = ""; 




  render() {

    const STR_TITLE = "Create";
    const STR_TITLETWO = "your own JIGs";
    const STR_SUBTITLE = "Learning Through Creation";
    const STR_LIBRARY = "Big content library";
    const STR_INTERFACE = "Smart & friendly interface";
    const STR_TEACHING = "Teaching through creation";
    const STR_ALL = "All in one";
    const STR_START ="Start creating";

    const {} = this;

 
     return html`
    <main>
    <div class="left_side">
      <img-ui path="girl@2x.jpg" class="girl"><img-ui>
    </div> 

    <div class="right_side">
    <sub-title size="medium" class="subtitle" title="${STR_SUBTITLE}"></sub-title>

     <div class="inside-wrapper">
     <title-section titlecolor="pink" title="${STR_TITLE}" size="large" class="title"></title-section>
    <title-section titlecolor="darkblue" title="${STR_TITLETWO}" size="large" class="title"></title-section>
    </div>
    
    <column-list text_line="${STR_LIBRARY}" size="medium" class="line"></column-list>
    <column-list text_line="${STR_INTERFACE}" size="medium" class="line"></column-list>
    <column-list text_line="${STR_TEACHING}" size="medium" class="line"></column-list>
    <column-list text_line="${STR_ALL}" size="medium" class="line"></column-list>

    <button-rect class="Start-creating" size="large"  color="red", bold="true" imglefthidden="false" imgrighthidden="true">${STR_START}</button-rect>
    <img-ui path="illustration-gears.png" class="gears"><img-ui>

    </div>


    </main>
  `;
  }
}