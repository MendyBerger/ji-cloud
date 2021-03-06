import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/entry/home/TOSORT/icon-wparagraph";
import "@elements/entry/home/sections/aboutus-section";
import "@elements/core/titles/variants/title-section";
import "@elements/core/titles/variants/subtitle";
import "@elements/entry/home/TOSORT/list-type";
import "@elements/core/dividers/circle-div";
import "@elements/core/titles/title-sub-paragraph";
import {Color, FontSize} from "@elements/core/titles/title-sub-paragraph";


  
export default {
  title: 'Entry/ Home / Section',
}

interface Args{
  color: Color,
  size: FontSize,
}

const DEFAULT_ARGS:Args = {
  color:"yellow",
  size: "medium",
}


const STR_STARTTITLE = "What they say ";
const STR_ENDTTITLE = "about us ";

const STR_TITLEPARENTS = "Parents";
const STR_SUBTITLEPARENTS = "Sarah Nazirah, Mexico";

const STR_TITLETEACHERS = "Teachers";
const STR_SUBTITLETEACHERS = "Sarah Nazirah, Mexico";

const STR_LINE1="I want to tell you, because of JI, my children are ";
const STR_LINE2="learning Hebrew and English simultaneously. For my ";
const STR_LINE3="children, you are not only teaching two children, you"
const STR_LINE4="are also saving their souls. I reaffirm that they have";
const STR_LINE5="achieved educational rehabilitation, thanks to JI.";


export const AboutUsParagraph = (props?:Partial<Args>) => {
  const {...titleProps} = props;
  props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `

    <aboutus-section>
   
    <title-section  title="${STR_STARTTITLE}"   slot="title"></title-section>
    <title-section titlecolor="yellow" title="${STR_ENDTTITLE}" size="large" slot="title"></title-section>
<title-sub-paragraph slot="title-sub-paragraph-right"   ${argsToAttrs(deleteNone(titleProps))}  colorsubtitle="black" sizesubtitle="small"  title="${STR_TITLEPARENTS}" subtitle="${STR_SUBTITLEPARENTS}">
 

<column-list slot="line" text_line="${STR_LINE1}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE2}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE3}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE4}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE5}" size="medium"></column-list>

</title-sub-paragraph>
   
<circle-div slot="rpoint" color="white" size="medium"></circle-div>
<circle-div slot="rpoint" color="pink" size="medium"></circle-div>
<circle-div slot="rpoint" color="pink" size="medium"></circle-div>
<circle-div slot="rpoint" color="pink" size="medium"></circle-div>
<circle-div slot="rpoint" color="pink" size="medium"></circle-div>
<img-ui slot="imgparent" path="sarah-nazirah.png"> </img-ui>


<title-sub-paragraph slot="title-sub-paragraph-left"   ${argsToAttrs(deleteNone(titleProps))}  colorsubtitle="black"   sizesubtitle="small" title="${STR_TITLETEACHERS}" subtitle="${STR_SUBTITLETEACHERS}">



<column-list slot="line" text_line="${STR_LINE1}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE2}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE3}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE4}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE5}" size="medium"></column-list>

</title-sub-paragraph>

<circle-div slot="lpoint" color="white" size="medium"></circle-div>
<circle-div slot="lpoint" color="pink" size="medium"></circle-div>
<circle-div slot="lpoint" color="pink" size="medium"></circle-div>
<circle-div slot="lpoint" color="pink" size="medium"></circle-div>
<circle-div slot="lpoint" color="pink" size="medium"></circle-div>
<img-ui slot="imgteacher" path="Sara-Halberstadt.png"> </img-ui>


    </aboutus-section>
    `
}
AboutUsParagraph.args = DEFAULT_ARGS;
