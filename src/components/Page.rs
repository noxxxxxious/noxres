use sycamore::prelude::*;

#[component]
pub fn Page<G: Html>() -> View<G> {

    let nav_ref_top = create_node_ref();
    let nav_ref_portfolio = create_node_ref();
    let nav_ref_about = create_node_ref();
    let nav_ref_profiles = create_node_ref();

    let nav_refs = [nav_ref_top, nav_ref_portfolio, nav_ref_about, nav_ref_profiles];

    view! {
        nav {
             ul {
                li(ref=nav_ref_top, class="active") { "Top" }
                li(ref=nav_ref_portfolio) { "Portfolio" }
                li(ref=nav_ref_about) { "About" }
                li(ref=nav_ref_profiles) { "Profiles" }
            }
        }
        crate::components::SectionTop::SectionTop(nav_refs=nav_refs.clone()) {}
        crate::components::SectionPortfolio::SectionPortfolio(nav_refs=nav_refs.clone()) {}
        crate::components::SectionAbout::SectionAbout(nav_refs=nav_refs.clone()) {}
        crate::components::SectionProfiles::SectionProfiles(nav_refs=nav_refs) {}
    }
}