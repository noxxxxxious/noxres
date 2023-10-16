use sycamore::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PortfolioItemProps {
    pub title: String,
    pub subtitle: String,
    pub img_href: String,
    pub img_alt: String,
    pub technologies: Vec<String>,
    pub description: String,
    pub github_href: Option<String>,
    pub preview_href: Option<String>
}

#[component]
pub fn PortfolioItem<G: Html>(props: PortfolioItemProps) -> View<G> {
    let technology_pills = View::new_fragment(
        props.technologies.into_iter().map(|t| view!{ (t) }).collect()
    );

    let preview_link = match props.preview_href {
        Some(link) => view! { a(href=link) { "View Project" } },
        None => view! { span(class="empty"){} }
    };

    let github_link = match props.github_href {
        Some(link) => view! { a(href=link) { "View Source" } },
        None => view! { span(class="empty"){} }
    };

    view! {
        article(class="portfolio-item--container") {
            div(class="portfolio-item--image") {
                img(src=props.img_href, alt=props.img_alt)
            }
            div(class="portfolio-item--info-wrapper") {
                div(class="portfolio-item--info--title-wrapper") {
                    div(class="portfolio-item--info--title") {(props.title)}
                    div(class="portfolio-item--info--subtitle") {(props.subtitle)}
                }
                div(class="portfolio-item--info--description") {(props.description)}
                div(class="portfolio-item--info--technologies") {(technology_pills)}
                div(class="portfolio-item--info--links-wrapper") {
                    (preview_link)
                    (github_link)
                }
            }
        }
    }
}
