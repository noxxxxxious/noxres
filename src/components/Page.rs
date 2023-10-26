use sycamore::prelude::*;
use web_sys::{Event, console, Document, Element};

#[component]
pub fn Page<G: Html>() -> View<G> {

    let main_ref = create_node_ref();
    let nav_refs: [NodeRef<G>; 4] = [create_node_ref(), create_node_ref(), create_node_ref(), create_node_ref()];
    let section_refs: [NodeRef<G>; 4] = [create_node_ref(),create_node_ref(),create_node_ref(),create_node_ref()];

    let scroll_handler = move |_event: Event| {
        let main = main_ref.get::<DomNode>().to_web_sys();
        for node in section_refs {
            let element = node.get::<DomNode>().to_web_sys();
        }
    };

    view! {
        main(ref=main_ref, on:scroll=scroll_handler) {
            nav {
                ul {
                    li(ref=nav_refs[0], class="active") { "Top" }
                    li(ref=nav_refs[1]) { "Portfolio" }
                    li(ref=nav_refs[2]) { "About" }
                    li(ref=nav_refs[3]) { "Profiles" }
                }
            }
            crate::components::SectionTop::SectionTop(in_ref=section_refs[0]) {}
            crate::components::SectionPortfolio::SectionPortfolio(in_ref=section_refs[1]) {}
            crate::components::SectionAbout::SectionAbout(in_ref=section_refs[2]) {}
            crate::components::SectionProfiles::SectionProfiles(in_ref=section_refs[3]) {}
        }
    }
}