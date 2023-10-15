use sycamore::prelude::*;

#[component]
pub fn SectionTop<G: Html>() -> View<G> {
    view! {
        section(id="top"){
            div(class="container landing--container"){
                div(class="landing--heading--container"){
                    span(class="landing--heading--name"){ "Sean Fried" }
                    span(class="landing--heading--title"){ "Front End Developer" }
                    span(class="landing--heading--subtitle"){ "Profile site unnecessarily rewritten in Rust!"}
                }
                div(class="landing--picture--container"){
                    div(class="landing--picture--image"){}
                }
            }
        }
    }
}
