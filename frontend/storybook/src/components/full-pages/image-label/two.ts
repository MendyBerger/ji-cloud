import "@elements/admin/templates-layout/image-label-full";
import { LeftLabel } from "~/components/admin/images/image-label-left";
import { TreeDropdown } from "~/components/dropdown";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-blue";
import "@elements/dropdowns/tree-dropdown-child";
import "@elements/cards/blue-card";
import {mockHierarchy, TreeNode} from "~/mock/hierarchy";
import { TitleWithInput } from "~/components/input";
import {mapToString} from "@utils/array";

export default {
  title: 'Full Pages/Image Label',
}


const leafNode = ({label, open, children}:TreeNode) => {
  return `
    <tree-dropdown-child label="${label}" ${open ? "open" : ""}>
    ${mapToString (children, leafNode)}
    </tree-dropdown-child>
  `;
}
const rootNode = ({label, open, children}:TreeNode) => {
  return `
    <tree-dropdown label="${label}" ${open ? "open" : ""}>
      ${mapToString (children, leafNode)}
    </tree-dropdown>
  `;
}

interface Props {
  title: string,
  label: string,
  titletwo: string,
  data: Array<TreeNode>
}

const DEFAULT_ARGS:Props = {
  title: "Label Images",
  label: "Category",
  titletwo: "Categories Summary",
  data: mockHierarchy
}
//To put in search-input: "search-24-px.svg",
export const ImageLabelFullTwo = (props?:Props) => {
  const { title, label, data, titletwo } = props || DEFAULT_ARGS;

  console.log(data);

  return `
    <imagelabel-full>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      
      <div slot="middle">
        <title-winput title="${title}">
          <search-input placeholder="${label}" slot="input">

          </search-input>
        </title-winput>
        ${data.map(rootNode)}
      </div>
      <div slot="right">
        <plain-blue title="${titletwo}"></plain-blue>
        <blue-card></blue-card>
      </div>
    </imagelabel-full>
    
    `
}

ImageLabelFullTwo.args = DEFAULT_ARGS;
ImageLabelFullTwo.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}