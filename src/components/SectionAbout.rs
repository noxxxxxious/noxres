use sycamore::prelude::*;

#[component(inline_props)]
pub fn SectionAbout<G: Html>(in_ref: NodeRef<G>) -> View<G> {
    view! {
        section(ref=in_ref, id="about") {
            div(class="header") { "About" }
            div(class="about--container") {
                div {
                    div(class="about--content-connector") {}
                    div(class="about--info-wrapper") {
                        div(class="about--info--history") {
                            div(class="about--info--header") {
                                "History"
                            }
                            div(class="about--info--underline")
                            div(class="about--info--hero") {
                                "Bramtech Solutions"
                            }
                            div(class="about--info--content") {
                                span { "2021 - Present"}
                                span { "Central Services & Support Specialist" }
                                ul{
                                    li {"PowerShell Automation"}
                                    li {"Windows Server Administration"}
                                    li {"Replibit and Shadowcopy Management"}
                                    li {"Workflow Tool Development using Web Technologies"}
                                }
                            }
                            div(class="about--info--divider-diamond"){
                                div(class="about--info--divider-diamond--line")
                            }
                            div(class="about--info--hero") {
                                "NOLA Web Hosting"
                            }
                            div(class="about--info--content") {
                                span { "2017 - 2020"}
                                span { "Front End Developer" }
                                ul{
                                    li {"Designing websites for local businesses"}
                                    li {"HTML & CSS, JavaScript, PHP, etc"}
                                    li {"Server Management & DNS Configuration"}
                                    li {"Project planning with clients"}
                                }
                            }
                        }
                    }
                    div(class="about--content-connector bottom")
                }
                div{
                    div(class="about--diamond") {
                        div(class="about--diamond--background")
                    }
                    div(class="about--content-connector") {}
                    div(class="about--info-wrapper") {
                        div(class="about--info--education") {
                            div(class="about--info--header") {
                                "Education"
                            }
                            div(class="about--info--underline")
                            div(class="about--info--hero") {
                                "Antonelli College"
                            }
                            div(class="about--info--content") {
                                span { "Business Technology & Networking"}
                                span { "Associate in Applied Business" }
                                span { "2009 - 2011" }
                            }
                        }
                        div(class="about--info--divider-diamond"){
                            div(class="about--info--divider-diamond--line")
                        }
                        div(class="about--info--header") {
                            "Certifications"
                        }
                        div(class="about--info--underline")
                        div(class="about--info--hero") {
                            "JLPT N3"
                        }
                        div(class="about--info--content") {
                            span { "2020"}
                            span { "Credential ID:" }
                            span { "4021401-30033" }
                        }
                        div(class="about--info--divider-diamond"){
                            div(class="about--info--divider-diamond--line")
                        }
                        div(class="about--info--hero") {
                            "JLPT N5"
                        }
                        div(class="about--info--content") {
                            span { "2016"}
                            span { "Credential ID:" }
                            span { "15B4024021401-300331501-50033" }
                        }
                    }
                    div(class="about--content-connector bottom")
                }
                div {
                    div(class="about--content-connector") {}
                    div(class="about--info-wrapper") {
                        div(class="about--info--education") {
                            div(class="about--info--header") {
                                "Skills"
                            }
                            div(class="about--info--underline")
                            div(class="about--info--hero") {
                                "Web Technologies"
                            }
                            div(class="about--info--content") {
                                div(class="about--info--skill-list"){
                                    span {"HTML"}
                                    span {"CSS"}
                                    span {"JavaScript"}
                                    span {"TypeScript"}
                                    span {"Vue"}
                                    span {"Nuxt"}
                                    span {"React"}
                                    span {"Next"}
                                    span {"MongoDB"}
                                    span {"Node"}
                                    span {"Express"}
                                    span {"REST"}
                                    span {"Electron"}
                                    span {"Vuetify"}
                                    span {"Bootstrap"}
                                    span {"jQuery"}
                                    span {"Rust"}
                                    span {"Sycamore"}
                                }
                            }
                            div(class="about--info--divider-diamond"){
                                div(class="about--info--divider-diamond--line")
                            }
                            div(class="about--info--hero") {
                                "Administrative"
                            }
                            div(class="about--info--content") {
                                div(class="about--info--skill-list"){
                                    span {"Windows"}
                                    span {"Linux"}
                                    span {"Windows Server"}
                                    span {"Active Directory"}
                                    span {"DNS"}
                                    span {"LDAP"}
                                    span {"VSS"}
                                    span {"Office 365"}
                                    span {"AzureAD"}
                                    span {"Exchange Online"}
                                    span {"PowerShell"}
                                }
                            }
                        }
                    }
                    div(class="about--content-connector bottom")
                }
            }
        }
    }
}
