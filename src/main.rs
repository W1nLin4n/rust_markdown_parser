use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"] // Path to the grammar file
struct MarkdownParser;

fn main() {
    let markdown = r#"
# Heading 1
## Heading 2

---

This is a paragraph with **bold text** and _italic text_.

- List item 1
- List item 2
>>
1. List item 3
2. List item 4

[Link](https://example.com)

```
Code block content
print("Hello world!")
```
"#;

    match MarkdownParser::parse(Rule::markdown, markdown) {
        Ok(mut parsed) => {
            // println!("Parsed successfully! Raw parse tree:");
            // for pair in parsed.clone() {
            //     println!("{:?}", pair);
            // }

            let html = parse_markdown(parsed.next().unwrap());
            println!("\nGenerated HTML:\n{}", html);
        }
        Err(e) => eprintln!("Error parsing Markdown: {}", e),
    }
}

fn parse_markdown(elem: pest::iterators::Pair<Rule>) -> String {
    let mut html = String::new();

    for pair in elem.into_inner() {
        match pair.as_rule() {
            Rule::header => {
                html.push_str(&parse_header(pair))
            }
            Rule::thematic_break => {
                html.push_str("<hr/>\n");
            }
            Rule::ordered_list => {
                html.push_str(&format!("<ol>\n{}</ol>\n", parse_list(pair)));
            }
            Rule::unordered_list => {
                html.push_str(&format!("<ul>\n{}</ul>\n", parse_list(pair)));
            }
            Rule::code_block => {
                html.push_str(&parse_code_block(pair));
            }
            Rule::paragraph => {
                html.push_str(&parse_paragraph(pair));
            }
            _ => {}
        }
    }

    html
}

fn parse_header(elem: pest::iterators::Pair<Rule>) -> String {
    let mut len = 0;
    let mut val = String::new();
    for pair in elem.into_inner() {
        match pair.as_rule() {
            Rule::header_hashtags => {
                len = pair.as_str().len()
            }
            Rule::line_pars => {
                val = parse_line_pars(pair)
            }
            _ => {}
        }
    }
    format!("<h{len}>{val}</h{len}>\n")
}

fn parse_list(elem: pest::iterators::Pair<Rule>) -> String {
    let mut items = String::new();
    for pair in elem.into_inner() {
        items.push_str(&format!("<li>{}</li>\n", parse_line_pars(pair)));
    }
    items
}

fn parse_code_block(elem: pest::iterators::Pair<Rule>) -> String {
    let mut rows = String::new();
    for pair in elem.into_inner() {
        rows.push_str(&format!("{}\n", pair.as_str()));
    }
    format!("<pre><code>\n{rows}</code></pre>\n")
}

fn parse_paragraph(elem: pest::iterators::Pair<Rule>) -> String {
    let mut rows = String::new();
    for pair in elem.into_inner() {
        rows.push_str(&format!("<p>{}</p>\n", parse_line_pars(pair)));
    }
    rows
}

fn parse_line_pars(elem: pest::iterators::Pair<Rule>) -> String {
    let mut line = String::new();
    for pair in elem.into_inner() {
        match pair.as_rule() {
            Rule::link => {
                line.push_str(&parse_link(pair));
            }
            Rule::bold => {
                line.push_str(&parse_bold(pair));
            }
            Rule::italic => {
                line.push_str(&parse_italic(pair));
            }
            Rule::text => {
                line.push_str(pair.as_str());
            }
            _ => {}
        }
    }
    line
}

fn parse_link(elem: pest::iterators::Pair<Rule>) -> String {
    let mut text = String::new();
    let mut href = String::new();
    for pair in elem.into_inner() {
        match pair.as_rule() {
            Rule::link_text => {
                text = pair.as_str().to_owned()
            }
            Rule::link_href => {
                href = pair.as_str().to_owned()
            }
            _ => {}
        }
    }
    format!("<a href=\"{href}\">{text}</a>")
}

fn parse_bold(elem: pest::iterators::Pair<Rule>) -> String {
    let text = elem.into_inner().next().unwrap().as_str();
    format!("<strong>{text}</strong>")
}

fn parse_italic(elem: pest::iterators::Pair<Rule>) -> String {
    let text = elem.into_inner().next().unwrap().as_str();
    format!("<em>{text}</em>")
}