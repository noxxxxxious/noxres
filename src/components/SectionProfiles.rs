use sycamore::prelude::*;

#[component]
pub fn SectionProfiles<G: Html>() -> View<G> {
    view! {
        section(id="profiles"){
            div(class="header"){ "Profiles" }
            div(class="profiles--container"){}
        }
    }
}
