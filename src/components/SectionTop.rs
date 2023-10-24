use sycamore::prelude::*;
use crate::scroller;

#[component]
pub fn SectionTop<G: Html>() -> View<G> {
    let top_ref = create_node_ref();

    on_mount(move || {
        let node = top_ref.get::<DomNode>();
        scroller::add_scroll_element(node);
    });

    view! {
        section(id="top", ref=top_ref){
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
