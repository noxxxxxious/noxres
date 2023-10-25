use sycamore::{prelude::*, web::html::ev::scroll};
use web_sys::{Event, console};

#[component(inline_props)]
pub fn SectionTop<G: Html>(nav_refs: [NodeRef<G>; 4]) -> View<G> {
    let section_ref = create_node_ref();
    
    let scroll_handler = move |event: Event| {
        console::log_1(&"text".into());
    };

    on_mount(move || {
        section_ref.get::<DomNode>().event(web_sys::WheelEvent, |event: Event|{
            if let Some(target) = event.target() {
                console::log_1(&target.js_typeof());
            }
            console::log_1(&"event text".into());
        });
        console::log_1(&"mount text".into());
    });

    view! {
        section(id="top", ref=section_ref){
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
