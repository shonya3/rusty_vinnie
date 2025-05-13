use scraper::ElementRef;
use std::fmt::Write;

pub fn clean_text(text: &str) -> String {
    text.replace("\t", "")
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn html_to_markdown(element: &ElementRef) -> String {
    let mut output = String::new();
    let mut in_list = false;

    for node in element.children() {
        match node.value() {
            scraper::node::Node::Text(t) => {
                let text = t.trim();
                if !text.is_empty() {
                    if in_list {
                        // Handle text nodes directly under ul/ol but not in li
                        in_list = false;
                        writeln!(&mut output).unwrap();
                    }
                    write!(&mut output, "{}", clean_text(text)).unwrap();
                }
            }
            scraper::Node::Element(e) => match e.name() {
                "br" => {
                    // Respect <br> tags as explicit newlines
                    writeln!(&mut output).unwrap();
                }
                "li" => {
                    in_list = true;
                    let li_text = ElementRef::wrap(node).unwrap().text().collect::<String>();
                    let cleaned = clean_text(&li_text);
                    writeln!(&mut output, "- {}", cleaned).unwrap();
                }
                "ul" | "ol" => {
                    let list_md = html_to_markdown(&ElementRef::wrap(node).unwrap());
                    write!(&mut output, "{}", list_md).unwrap();
                    in_list = false
                }
                "h1" => {
                    let text = ElementRef::wrap(node).unwrap().text().collect::<String>();
                    writeln!(&mut output, "# {}", clean_text(&text)).unwrap();
                }
                "h2" => {
                    let text = ElementRef::wrap(node).unwrap().text().collect::<String>();
                    writeln!(&mut output, "## {}", clean_text(&text)).unwrap();
                }
                "h3" => {
                    let text = ElementRef::wrap(node).unwrap().text().collect::<String>();
                    writeln!(&mut output, "### {}", clean_text(&text)).unwrap();
                }
                "strong" => {
                    let text = ElementRef::wrap(node).unwrap().text().collect::<String>();
                    write!(&mut output, "**{}**", clean_text(&text)).unwrap();
                }
                "a" => {
                    let el = ElementRef::wrap(node).unwrap();
                    let text: String = el.text().collect();
                    let text = clean_text(&text);

                    match el.attr("href") {
                        Some(href) => write!(&mut output, " [{text}]({href}) ").unwrap(),
                        None => write!(&mut output, "{text}").unwrap(),
                    };
                }
                _ => {
                    if in_list {
                        in_list = false;
                        writeln!(&mut output).unwrap();
                    }
                    let child_text = html_to_markdown(&ElementRef::wrap(node).unwrap());
                    write!(&mut output, "{}", child_text).unwrap();
                }
            },
            _ => {}
        }
    }

    output
}
