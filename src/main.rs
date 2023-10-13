use sycamore::prelude::*;

fn main() {
    sycamore::render(|| view! {
        nav {
             ul {
                li(class="active") { "Top" }
                li { "Portfolio" }
                li { "About" }
                li { "Profiles" }
            }
        }
        main { "Hello" }
    });
}
