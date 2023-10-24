use sycamore::prelude::*;
use log::Level;
mod components;
mod scroller;

fn main() {
    console_log::init_with_level(Level::Debug);

    let sections: Vec<DomNode> = Vec::new();


    sycamore::render(|| view! {
        nav {
             ul {
                li(class="active") { "Top" }
                li { "Portfolio" }
                li { "About" }
                li { "Profiles" }
            }
        }
        components::SectionTop::SectionTop {}
        components::SectionPortfolio::SectionPortfolio {}
        components::SectionAbout::SectionAbout {}
        components::SectionProfiles::SectionProfiles {}
    });
}
