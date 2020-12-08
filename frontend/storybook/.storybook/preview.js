
import addons from '@storybook/addons';
import { STORY_RENDERED } from '@storybook/core-events';
import {startResizer} from "../src/utils/resize";
import {getChildId} from "../src/utils/dom";

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
}

let lastCancel = null;

addons.getChannel().on(STORY_RENDERED, (story) => {
  if(lastCancel != null) {
    lastCancel();
  }
  const container = getChildId(document, "module-outer", true);
  const [observer, cancelObserver] = startResizer(container);

  const sidebar = getChildId(document, "sidebar");
  const header = getChildId(document, "header");
  const footer = getChildId(document, "footer");

  if(sidebar) {
    observer.observe(sidebar);
  }
  if(header) {
    observer.observe(header);
  }
  if(footer) {
    observer.observe(footer);
  }

	//Note - not observing size changes on main
	//Main is ultimately what's scaled :)


  lastCancel = cancelObserver;
});