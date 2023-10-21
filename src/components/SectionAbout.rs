use sycamore::prelude::*;

#[component]
pub fn SectionAbout<G: Html>() -> View<G> {
    view! {
        section(id="about"){
            div(class="header"){ "About" }
            div(class="about--container"){
                div(class="about--diamond") {
                    div(class="about--diamond--background"){}
                }
                div(class="about--content-connector") {}
                div(class="about--info-wrapper") {
                    div(class="about--info--education") {
                        div(class="about--info--header") {
                            "Education"
                        }
                        div(class="about--info--content") {
                            "Antonelli College"
                        }
                    }
                }
                div(class="about--content-connector bottom") {}
            }
        }
    }
}
