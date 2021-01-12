use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

const PLAYER:&'static str = "player";
const CARD:&'static str = "card";

pub fn player() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(PLAYER))
}


pub fn card() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD))
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
            (PLAYER, include_str!("../../../../../../../.template_output/module/memory/play/player.html")),
            (CARD, include_str!("../../../../../../../.template_output/module/memory/play/memory-card.html")),
        ]);

        Self { cache }
    }

}