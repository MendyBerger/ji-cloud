use url::Url;
use crate::locale::state::{EntryStatus, ItemKind, Entry, State, Section, Column};
use web_sys::HtmlSelectElement;
use std::rc::Rc;
use std::clone::Clone;
use wasm_bindgen::prelude::*;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::SignalVecExt;
use dominator::{Dom, html, clone, events, with_node};
use std::str::FromStr;
use strum::IntoEnumIterator;


#[derive(Clone)]
pub struct EntryRow {

}

impl EntryRow {
    fn url_option_string(url: &Option<Url>) -> String {
        if url.is_some() {
            url.clone().unwrap().to_string()
        } else {
            String::new()
        }
    }

    pub fn render(entry: Rc<Mutable<Entry>>, state: Rc<State>) -> Dom {
        html!("locale-row", {
            .property("slot", "rows")
            .children_signal_vec(state.visible_columns.signal_vec_cloned()
                .map(clone!(state, entry => move |column| {
                    let entry_ref = entry.lock_ref();
                    match column {
                        Column::ID => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .property("value", &entry_ref.id)
                                    .event(clone!(entry => move |event: events::Input| {
                                        let value: String = event.value().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.id = value;
                                    }))
                                }))
                            })
                        },
                        Column::Section => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .apply_if(entry_ref.section.is_some(), |dom| {
                                        dom.property("value", &entry_ref.section.clone().unwrap())
                                    })
                                    .attribute("list", "sections")
                                    .event(clone!(entry => move |event: events::Input| {
                                        let value: Section = event.value().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.section = Some(value);
                                    }))
                                    .event(clone!(state => move |_: events::Change| {
                                        state.regenerate_section_options();
                                    }))
                                }))
                            })
                        },
                        Column::ItemKind => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .apply_if(entry_ref.item_kind.is_some(), |dom| {
                                        dom.property("value", &entry_ref.item_kind.clone().unwrap())
                                    })
                                    .attribute("list", "item-kinds")
                                    .event(clone!(entry => move |event: events::Input| {
                                        let value: ItemKind = event.value().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.item_kind = Some(value);
                                    }))
                                    .event(clone!(state => move |_: events::Change| {
                                        state.regenerate_item_kind_options();
                                    }))
                                }))
                            })
                        },
                        Column::English => {
                            html!("locale-cell", {
                                .child(html!("textarea", {
                                    .text(&entry_ref.english)
                                    .event(clone!(entry => move |event: events::Input| {
                                        let value: String = event.value().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.english = value;
                                    }))
                                }))
                            })
                        },
                        Column::Hebrew => {
                            html!("locale-cell", {
                                .child(html!("textarea", {
                                    .text(&entry_ref.hebrew)
                                    .event(clone!(entry => move |event: events::Input| {
                                        let value: String = event.value().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.hebrew = value;
                                    }))
                                }))
                            })
                        },
                        Column::Status => {
                            html!("locale-cell", {
                                .child(html!("select" => HtmlSelectElement, {
                                    .with_node!(elem => {
                                        .event(clone!(entry => move |_event: events::Change| {
                                            let value: String = elem.value();
                                            let mut entry = entry.lock_mut();
                                            entry.status = EntryStatus::from_str(&value).unwrap_throw();
                                        }))
                                    })
                                    .children(
                                        EntryStatus::iter().map(|o| {
                                            html!("option", {
                                                .property("text", o.to_string())
                                                .property("value", o.to_string())
                                                .property("selected", o == entry_ref.status)
                                            })
                                        })
                                    )
                                }))
                            })
                        },
                        Column::ZeplinReference => {
                            html!("locale-cell", {
                                .child(html!("locale-hover-link", {
                                    .property_signal("link", entry_ref.zeplin_reference.signal_ref(|url| Self::url_option_string(url)))
                                    .child(html!("input", {
                                        .property("type", "url")
                                        .apply_if(entry_ref.zeplin_reference.lock_ref().is_some(), |dom| {
                                            dom.property("value", &entry_ref.zeplin_reference.lock_ref().clone().unwrap().to_string())
                                        })
                                        .event(clone!(entry => move |event: events::Input| {
                                            let value: String = event.value().unwrap_throw();
                                            let value = Url::parse(&value);

                                            let zeplin_reference = &entry.lock_ref().zeplin_reference;
                                            match value {
                                                Ok(value) => zeplin_reference.set(Some(value)),
                                                Err(_) => zeplin_reference.set(None),
                                            };
                                        }))
                                    }))
                                }))
                            })
                        },
                        Column::Comments => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .property("value", &entry_ref.comments)
                                    .event(clone!(entry => move |event: events::Input| {
                                        let value: String = event.value().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.comments = value;
                                    }))
                                }))
                            })
                        },
                        Column::App => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .attribute("type", "checkbox")
                                    .property("checked", entry_ref.in_app)
                                    .event(clone!(entry => move |event: events::Change| {
                                        let value: bool = event.checked().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.in_app = value;
                                    }))
                                }))
                            })
                        },
                        Column::Element => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .attribute("type", "checkbox")
                                    .property("checked", entry_ref.in_element)
                                    .event(clone!(entry => move |event: events::Change| {
                                        let value: bool = event.checked().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.in_element = value;
                                    }))
                                }))
                            })
                        },
                        Column::Mock => {
                            html!("locale-cell", {
                                .child(html!("input", {
                                    .attribute("type", "checkbox")
                                    .property("checked", entry_ref.in_mock)
                                    .event(clone!(entry => move |event: events::Change| {
                                        let value: bool = event.checked().unwrap_throw();
                                        let mut entry = entry.lock_mut();
                                        entry.in_mock = value;
                                    }))
                                }))
                            })
                        },
                        Column::Actions => {
                            html!("locale-cell", {
                                .child(
                                    html!("locale-actions-wrapper", {
                                        .children(&mut [
                                            html!("button-text", {
                                                .property("slot", "first")
                                                .text("Clone")
                                                .event(clone!(state, entry => move |_event: events::Click| {
                                                    state.loader.load(clone!(state, entry => async move {
                                                        state.clone_entry(&entry.lock_ref()).await;
                                                    }))
                                                }))
                                            }),
                                            html!("button-text", {
                                                .property("slot", "second")
                                                .text("Delete")
                                                .event(clone!(state, entry => move |_event: events::Click| {
                                                    state.remove_entry(&entry.lock_ref().id);
                                                }))
                                            }),
                                        ])
                                    })
                                )
                            })
                        }
                    }
                }))
            )
            .event(clone!(state, entry => move |_event: events::Input| {
                state.saving_loader.load(clone!(state, entry => async move {
                    let entry = &entry.lock_ref().clone();
                    state.save_entry(entry).await;
                }))
            }))
        })
    }
}
