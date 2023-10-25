use sycamore::prelude::*;

#[component(inline_props)]
pub fn SectionProfiles<G: Html>(nav_refs: [NodeRef<G>; 4]) -> View<G> {
    view! {
        section(id="profiles"){
            div(class="header"){ "Profiles" }
            div(class="profiles--container"){
                a(href="https://github.com/noxxxxxious", class="profiles--link") {
                    img(src="images/github.png", alt="Github Logo")  
                }
                a(href="https://linkedin.com/in/seanmwfried", class="profiles--link") {
                    img(src="images/linkedin.svg", alt="LinkedIn Logo")  
                }
                div(class="no-email") {
                    div(class="no-email--header") { "Why no email?"}
                    div(class="about--info--divider-diamond no-email--divider"){
                        div(class="about--info--divider-diamond--line no-email--divider")
                    }
                    div(class="no-email--content") { "Let's be honest: If you email me with a job offer out of nowhere, I'm probably going to think that you are just trying to steal my information. Everyone has a LinkedIn now, and it offers much more peace of mind. If you would like to start a dialogue, please message me there." }
                    div(class="no-email--thanks") { 
                        span{ "Thank you for reading!" }
                        span{ "-Sean" }
                    }
                }
            }
        }
    }
}
