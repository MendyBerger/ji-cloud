use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use core::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

const CATEGORIES:&'static str = "categories";
const CATEGORY_MAIN_SELECTED:&'static str = "category_main_selected";
const CATEGORY_MAIN_DESELECTED:&'static str = "category_main_deselected";
const CATEGORY_SUB:&'static str = "category_sub";
const CATEGORY_LABEL_DISPLAY:&'static str = "category_label_display";
const CATEGORY_LABEL_INPUT:&'static str = "category_label_input";

pub fn categories() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORIES))
}
pub fn category_main(id:&str, selected:bool) -> HtmlElement {
    if selected {
        TEMPLATES.with(|t| t.cache.render_elem(CATEGORY_MAIN_SELECTED, &html_map!(
            "id" => id,
        )).unwrap())
    } else {
        TEMPLATES.with(|t| t.cache.render_elem(CATEGORY_MAIN_DESELECTED, &html_map!(
            "id" => id,
        )).unwrap())
    }
}

pub fn category_sub(id:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(CATEGORY_SUB, &html_map!(
        "id" => id,
    )).unwrap())
}
pub fn category_label_input() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORY_LABEL_INPUT))
}
pub fn category_label_display() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORY_LABEL_DISPLAY))
}


pub struct Templates {
    pub cache: TemplateCache<'static>
}

impl fmt::Debug for Templates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        f.debug_list()
            .entries(self.cache.templates.keys())
         .finish()
    }
}
impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (CATEGORIES, get_template_str(include_str!("../../../.template_output/categories/categories-page.html"))),
            (CATEGORY_MAIN_SELECTED, get_template_str(include_str!("../../../.template_output/categories/category_main_selected.html"))),
            (CATEGORY_MAIN_DESELECTED, get_template_str(include_str!("../../../.template_output/categories/category_main_deselected.html"))),
            (CATEGORY_SUB, get_template_str(include_str!("../../../.template_output/categories/category_sub.html"))),
            (CATEGORY_LABEL_DISPLAY, get_template_str(include_str!("../../../.template_output/categories/category_label_display.html"))),
            (CATEGORY_LABEL_INPUT, get_template_str(include_str!("../../../.template_output/categories/category_label_input.html"))),
        ]);

        Self { cache }
    }

}

//replace %MEDIA_UI% in the template string
//this leaks memory - which is okay since templates exist for the lifetime of the app
fn get_template_str(s:&'static str) -> &'static str {
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
