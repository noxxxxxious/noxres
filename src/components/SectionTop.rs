use sycamore::prelude::*;

#[component(inline_props)]
pub fn SectionTop<G: Html>(in_ref: NodeRef<G>) -> View<G> {
    view! {
        section(ref=in_ref, id="top"){
            div(class="container landing--container"){
                div(class="landing--heading--container"){
                    span(class="landing--heading--name"){ "Sean Fried" }
                    span(class="landing--heading--title"){ "Front End Developer" }
                    span(class="landing--heading--subtitle"){ "Vue ┃ Nuxt ┃ MongoDB ┃ Express ┃ Node"}
                }
                div(class="landing--picture--container"){
                    div(class="landing--picture--image"){}
                }
                div(class="landing--rust"){ 
                    div{"Rewritten"}
                    div{"in Rust!"}
                }
            }
        }
    }
}
