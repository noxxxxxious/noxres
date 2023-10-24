use log::info;
use sycamore::prelude::*;

pub fn on_scroll() {

}

pub fn add_scroll_element(in_element: DomNode) {
    let value = in_element.to_web_sys().text_content().unwrap();
    info!("{}", value);
    &SECTIONS.push(in_element);
}