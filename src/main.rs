use sycamore::prelude::*;
mod components;
// mod scroller;

fn main() {
    sycamore::render(|| view! {
        components::Page::Page{}
    });
}
