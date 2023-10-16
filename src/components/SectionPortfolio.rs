use sycamore::prelude::*;
use crate::components::PortfolioItem;

#[component]
pub fn SectionPortfolio<G: Html>() -> View<G> {

    let pitems = vec![
        PortfolioItem::PortfolioItemProps {
            title: "Simple IP (sip)".to_string(),
            subtitle: "Get your local IP with less garble.".to_string(),
            img_href: "images/headshot.jpg".to_string(),
            img_alt: "Screenshot of the sip command line utility.".to_string(),
            technologies: vec![
                "Rust".to_string(), 
                "Interfaces.rs".to_string()
            ],
            description: "A simple Rust command line utility that gives your local interface IP's and subnets without having to look through the mess that ip or ifconfig splat on your screen.".to_string(),
            github_href: Some("".to_string()),
            preview_href: None
        },
        PortfolioItem::PortfolioItemProps {
            title: "Yomikku".to_string(),
            subtitle: "A manga reader for Japanese learners.".to_string(),
            img_href: "images/headshot.jpg".to_string(),
            img_alt: "Screenshot of the Yomikku web application.".to_string(),
            technologies: vec![
                "Vue 3".to_string(), 
                "TypeScript".to_string(), 
                "MongoDB".to_string(),
                "Express".to_string(),
                "HTMLCanvas".to_string()
            ],
            description: "The first project I consulted a lot with ChatGPT on. It consists of two sides: Creator and Reader. Reader is the user-facing project that allows users to read manga using images that are drawn over with crisp text instead of having to rely on shoddy scans. In addition, clicking on a word will show the reading, part of speech, and definitions to help hide ignoring kanji and reading furigana, and also to cut out the need for leaving the page to look up a word. Creator, on the other hand, is a tool designed to make easier the process of creating the JSON's needed to display text bubbles on the pages. Instead of writing JSON by hand, one can draw lines and write text in a much more typical way.".to_string(),
            github_href: Some("".to_string()),
            preview_href: None
        }
    ];

    let pviews = View::new_fragment(
        pitems.into_iter().map(|item| view! { 
            PortfolioItem::PortfolioItem(
                title = item.title,
                subtitle = item.subtitle,
                img_href = item.img_href,
                img_alt = item.img_alt,
                technologies = item.technologies,
                description = item.description,
                github_href = item.github_href,
                preview_href = item.preview_href
            )
        }).collect()
    );

    view! {
        section(id="portfolio"){
            div(class="portfolio--container") {
                (pviews)
            }   
        }
    }
}
