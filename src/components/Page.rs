use sycamore::prelude::*;
use web_sys::{Event, window, Element};

#[component]
pub fn Page<G: Html>() -> View<G> {

    let main_ref = create_node_ref();
    let nav_refs: [NodeRef<G>; 4] = [create_node_ref(), create_node_ref(), create_node_ref(), create_node_ref()];
    let section_refs: [NodeRef<G>; 4] = [create_node_ref(),create_node_ref(),create_node_ref(),create_node_ref()];

    let scroll_handler = move |_event: Event| {
        if let Some(win) = window() {
            let v_threshold = win.inner_height().unwrap().unchecked_into_f64() / 2 as f64;
            for (index, node) in section_refs.iter().enumerate() {
                let element = node.get::<DomNode>().unchecked_into::<Element>();
                let bounds = element.get_bounding_client_rect().top();
                if bounds < v_threshold {
                    change_active(index, &nav_refs);
                    break;
                }
            }
        }
    };

    fn change_active<G: GenericNode>(active_index: usize, refs: &[NodeRef<G>; 4]) {
        for (index, other_node) in refs.iter().enumerate() {
            if index == active_index {
                other_node.get::<DomNode>().add_class("active");
            } else {
                other_node.get::<DomNode>().remove_class("active");
            }
        }

    }

    view! {
        main(ref=main_ref, on:scroll=scroll_handler) {
            nav {
                menu {
                    a(ref=nav_refs[3], class="active", href="#top") { span { "Top" } }
                    a(ref=nav_refs[2], href="#portfolio-anchor") { span { "Portfolio" } }
                    a(ref=nav_refs[1], href="#about-anchor") { span { "About" } }
                    a(ref=nav_refs[0], href="#profiles-anchor") { span { "Profiles" } }
                }
            }
            crate::components::SectionTop::SectionTop(in_ref=section_refs[3]) {}
            crate::components::SectionPortfolio::SectionPortfolio(in_ref=section_refs[2]) {}
            crate::components::SectionAbout::SectionAbout(in_ref=section_refs[1]) {}
            crate::components::SectionProfiles::SectionProfiles(in_ref=section_refs[0]) {}
        }
    }
}