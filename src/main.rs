use ibex::prelude::*;
use ibex::{routes, ssg};

const URL_ROOT: &str = "/";

fn main() {
    let routes = routes! [
        (/)    => at_index(),
        (/404) => at_404(),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn at_index() -> Document {
    document! { [lang="en"]
        @use_base []
        div ."header" {
            h1 { "darcy's website" }
            p {
                span { &laquo }
                "Welcome to my website."
                span { &raquo }
            }
        }

        main ."highlight-links" {
            article { h2 #"about" { "About Me" }
                div ."center" {
                    "I like programming and languages."
                    ~"I speak English, Esperanto, and Rust."
                }
            }
            
            hr/
            article { h2 #"links" { "Links" }
                div ."center" {
                    ul {
                        li { a [href="https://github.com/dxrcy"] { "My GitHub Profile" } }
                    }
                }
            }

            hr/
            article { h2 #"experience" { "Programming Experience" }
                h3 { "Rust" }
                p { "Rust is my favourite language, due to its type system, macros, and speed." }
                ul {
                    li { "CLI apps and simple tools" }
                    li { "Static site generation frameworks" }
                    li { "Procedural macros" }
                    li { "Egui (UI framework)" }
                    li { "GGez (Graphics framework)" }
                }
                h3 { "Javascript / Typescript" }
                ul {
                    li { "Client-side vanilla javascript with HTML and CSS or SCSS" }
                    li { "Server-side Node.JS, Express" }
                    li { "React single page applications" }
                }
                h3 { "Other" }
                ul {
                    li { "Posix-compliant shell scripting (like Bash)" }
                    li { "Popular CLI tools, like coreutils, FFmpeg, etc." }
                    li { "Some Lua, Haskell, and Handlebars" }
                    li { "Previously, Python and AutoHotkey, but I have not used them in a while." }
                }
                p {
                    "I am currently learning Zig."
                    ~ "I also plan to learn Go and C."
                }
            }

            hr/
            article { h2 #"projects" { "Projects" }
                ul ."big-list" {
                    li { h3 { em{"EveryGarf"} ~ "- Download every"~i{"Garfield"}~"comic as an image" }
                        p {
                            "A CLI tool which scrapes" ~ a [href="https://gocomics.com"] {i{"GoComics.com"}}
                            ~ "to download every"~i{"Garfield"}~"comic concurrently."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://github.com/dxrcy/everygarf"]
                                { "EveryGarf on GitHub" }
                            // ", or"
                            // ~ a [href="https://crates.io/crates/everygarf"]
                            //     { "on Crates.io" }
                        }
                    }
                    li { h3 { em{"Ibex"} ~ "- Static site generation framework for Rust" }
                        p {
                            "Write HTML-style templates, which compile to static HTML files."
                            ~ "Perfect for sites without dynamic content, that are only changed occasionally."
                            ~ "This website is actually written in using Ibex."
                            ~ "Similar to" ~ a [href="https://handlebarsjs.com"] {"Handlebars.js"}
                            ~ ", but type-safe!"
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://github.com/dxrcy/ibex"]
                                { "Ibex on GitHub" }
                            ","
                            ~ a [href="https://github.com/dxrcy/ibex-template"]
                                { "a basic SSG template" }
                           ", or"
                            ~ a [href="https://github.com/dxrcy/dxrcy.dev"]
                                { "the source code for"~b{"this"}~"website" }
                        }
                    }
                    li { h3 { em{"CTTab"} ~ "- A customizable 'new tab' page for the browser" }
                        p {
                            "Add a background, shortcuts, and notes to your browser's homepage."
                            ~ "Supports Chromium and Firefox based browsers."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://github.com/dxrcy/cttab"]
                                { "CTTab on GitHub" }
                           ", or"
                            ~ a [href="https://dxrcy.dev/cttab"]
                                { "a live example" }
                        }
                    }
                    li { h3 { em{"Phonet"} ~ "- Declarative"
                            ~ a [href="https://en.wikipedia.org/wiki/Phonotactics"] {"phonotactics"}
                            ~ "validation, using Regex"
                        }
                        p {
                            "Used to create constructed lanugages (conlangs)."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://github.com/dxrcy/phonet"]
                                { "Phonet on GitHub" }
                            ", or on"
                            ~ a [href="https://crates.io/crates/phonet"]
                                { "Crates.io" }
                        }
                    }
                    li { h3 { em{"Garf-EO"} ~ "- Garfield comics in Esperanto" }
                        p {
                            "500+ comics translated to Esperanto by your's truly."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://dxrcy.dev/garfeo"]
                                { "Garfield Esperanto (website)" }
                            ", or"
                            ~ a [href="https://github.com/dxrcy/garfeo"]
                                { "the source code" }
                        }
                    }
                    li { h3 { em{"The Trustworthy Times"} ~ "- A satirical news website (funny)" }
                        p {
                            "A silly little static website with funny fake news articles."
                            ~ "I might one day port it to"~i{"Ibex"}"."
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://dxrcy.dev/trustworthytimes"]
                                { i{"The Trustworthy Times"} }
                            ", or"
                            ~ a [href="https://github.com/dxrcy/trustworthytimes"]
                                { "the source code" }
                        }
                    }
                    li { h3 { em{"'Apple'"} ~ "- Breakthrough innovation in the fields of web design and the contemporary arts" }
                        p {
                            "As my most ambitious project yet, this website combines both cutting-edge"
                            ~ "technology and artistic expression"
                        }
                        blockquote {
                            "Check out"
                            ~ a [href="https://dxrcy.dev/apple"]
                                { "'Apple'" }
                            ", or"
                            ~ a [href="https://github.com/dxrcy/apple"]
                                { "the source code" }
                        }
                    }
                }

                h3 #"other-projects" { "Other projects" }
                p { "Which are either unmaintained or less interesting." }
                ul ."small-list" {
                    li { a [href="https://dxrcy.dev/color"]
                        { "Colour Sliders" }
                    }
                    li { a [href="https://github.com/dxrcy/recipe-lang"]
                        { "Programming language written like a cooking recipe" }
                    }
                    li { a [href="https://github.com/dxrcy/mcimg"]
                        { "Convert pixels of an image into"~i{"Minecraft"}~"blocks" }
                    }
                    li { a [href="https://github.com/dxrcy/unreact"]
                        { i{"Unreact"}~"- An old SSG framework using Handlebars. Predecesor to"~i{"Ibex"}"" }
                    }
                    li { a [href="https://github.com/dxrcy/lisp"]
                        { "Simple"~i{"Lisp"}"-like programming language" }
                    }
                    li { a [href="https://github.com/dxrcy/scripts"]
                        { "Some POSIX-compliant shell scripts" }
                    }
                }
            }

            hr/
            article { h2 #"workflow" { "Workflow" }
                p {
                    "All my"~i{"dotfiles"}~"are available"
                    ~a[href="https://github.com/dxrcy/dotfiles"]{"here"}
                    ", if you are interested."
                }
                p { strong{"Operating System:"} ~ "EndevourOS (Arch-based Linux), with i3 Window Manager." }
                p { strong{"Programming:"}      ~ "Tmux + Neovim + Git" }
            }
        }

        footer {
            "Thanks for checking out my website!"
        }
    }
    .into()
}

fn at_404() -> Document {
    document! { [lang="en"]
        @use_base []
        div ."center header" {
            h1 { "darcy's website" }
            h2 { "404 - Not found" }
            p {
                a [href=url!()] { "Did you mean to go the main page?" }
            }
        }
    }
    .into()
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()
                .url(url!())
                .title("darcy's website")
                .desc("Welcome to my website.")
                .image(url!("static/icon.png"))
                .author("darcy")
                .color("#86b1b0")
            ]
            title { "darcy's website" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
            @ssg::use_autoreload []
        }
    }
}

