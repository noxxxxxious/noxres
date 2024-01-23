use sycamore::prelude::*;
use crate::components::PortfolioItem;

#[component(inline_props)]
pub fn SectionPortfolio<G: Html>(in_ref: NodeRef<G>) -> View<G> {

    let pitems = vec![
        PortfolioItem::PortfolioItemProps {
            title: "Yomikku".to_string(),
            subtitle: "A manga reader for Japanese learners.".to_string(),
            img_href: "images/yomikku.jpg".to_string(),
            img_alt: "Screenshot of the Yomikku web application.".to_string(),
            technologies: vec![
                "Vue 3".to_string(), 
                "TypeScript".to_string(), 
                "MongoDB".to_string(),
                "Express".to_string(),
                "HTMLCanvas".to_string()
            ],
            description: "A WIP, my current project consists of two parts: Creator and Reader. Reader is the user-facing project that allows reading manga using drawn over images with crisp text instead of shoddy scans, as well as seeing word definitions, etc. Creator is a tool to make easier the process of creating the JSON's required for Reader. Instead of writing JSON by hand, one can draw lines and write text in a much more typical way.".to_string(),
            github_href: Some("".to_string()),
            preview_href: None,
        },
        PortfolioItem::PortfolioItemProps {
            title: "Jungle Camp".to_string(),
            subtitle: "Item information for the MOBA Predecessor.".to_string(),
            img_href: "images/junglecamp.png".to_string(),
            img_alt: "Screenshot of the Jungle Camp website.".to_string(),
            technologies: vec![
                "Vue 2".to_string(), 
                "Vuex".to_string(), 
            ],
            description: "During the alpha and beta, there were no resources to be able to look at the game items outside of the game, in particular in-between alpha tests. I created Jungle Camp as a way for people to access the item information and theorycraft while being unable to play. Eventually, I joined a team of people and worked on a different (also now defunct) project, and Jungle Camp was gobbled up by it. Thus, the data is out of date and put it back online solely for portfolio purposes.".to_string(),
            github_href: Some("".to_string()),
            preview_href: Some("".to_string()),
        },
        PortfolioItem::PortfolioItemProps {
            title: "Rust Portfolio Website".to_string(),
            subtitle: "Did I mention it was written in Rust?".to_string(),
            img_href: "images/noxres.jpg".to_string(),
            img_alt: "Screenshot of this website.".to_string(),
            technologies: vec![
                "Rust".to_string(), 
                "Sycamore".to_string(), 
            ],
            description: "I've remade my portfolio website a few different times with pure HTML, CSS, and vanilla JavaScript. I remade it using Vue, and was working on remaking it again using NextJS; however, I felt like I wanted to do something different this time around and use something I was wholly unfamiliar with. After learning a bit of Rust and finding Sycamore, I decided to give it a shot!".to_string(),
            github_href: Some("".to_string()),
            preview_href: Some("".to_string()),
        },
        PortfolioItem::PortfolioItemProps {
            title: "Simple IP (sip)".to_string(),
            subtitle: "For when you only need a sip of IP info.".to_string(),
            img_href: "images/sip.jpg".to_string(),
            img_alt: "Screenshot of the sip command line utility.".to_string(),
            technologies: vec![
                "Rust".to_string(), 
                "Interfaces.rs".to_string()
            ],
            description: "A simple command line utility that gives your local interface IP's and subnets without having to look through the mess that ip or ifconfig splat on your screen.".to_string(),
            github_href: Some("".to_string()),
            preview_href: None
        },
        PortfolioItem::PortfolioItemProps {
            title: "Hudu Keyboard Navigation".to_string(),
            subtitle: "Hu needs a mouse anyway?".to_string(),
            img_href: "images/tampermonkey.png".to_string(),
            img_alt: "Tampermonkey Logo".to_string(),
            technologies: vec![
                "JavaScript".to_string(), 
                "Tampermonkey".to_string(), 
            ],
            description: "I had put in a feature request to the Hudu Documentation team to support keyboard navigation a la IT Glue. The request never gained traction, so I took things into my own hands. I have actually become so accustomed to this that when I don't have it installed on one of the computers I'm using, I'll stop what I'm doing to install Tampermonkey and this userscript. It has become a necessity to myself and a couple others at my place of work.".to_string(),
            github_href: Some("".to_string()),
            preview_href: None
        },
        PortfolioItem::PortfolioItemProps {
            title: "Sycamore End Tags".to_string(),
            subtitle: "No more losing track of closing braces".to_string(),
            img_href: "images/sycamoreendtags.png".to_string(),
            img_alt: "Screenshot of Sycamore End Tags being used in Neovim.".to_string(),
            technologies: vec![
                "Lua".to_string(), 
                "Treesitter".to_string(), 
            ],
            description: "One of the things I love about Sycamore is the way it handles HTML as regular functions in the view macro; the only downside was occasionally losing track of which closing brace goes to which element. With this Neovim plugin, anything that has both parenthesis and braces will now receive virtual text to assist with keeping track of what brace closes which function block.".to_string(),
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
        section(ref=in_ref, id="portfolio"){
            div(id="portfolio-anchor", class="section-anchor") {}
            div(class="header") { 
                span(class="header--text"){ "Portfolio" }
            }
            div(class="portfolio--container") {
                (pviews)
            }   
        }
    }
}
