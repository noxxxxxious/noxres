use sycamore::prelude::*;
mod components;

fn main() {
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
