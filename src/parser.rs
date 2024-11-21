use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use std::{fs::File, io::Read};

#[derive(Parser)]
#[grammar = "markdown.pest"]
struct MarkdownParser;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Markdown parse error: {0}")]
    MarkdownParseError(String),
    #[error("Error while trying to open {0}")]
    FileOpenError(String),
    #[error("Error while trying to read {0}")]
    FileReadError(String),
}

pub fn markdown_file_to_html(path: String) -> Result<String> {
    let mut file = match File::open(path.clone()) {
        Err(_) => Err(ParserError::FileOpenError(path.clone()))?,
        Ok(file) => file
    };
    
    let mut markdown = String::new();
    match file.read_to_string(&mut markdown) {
        Err(_) => Err(ParserError::FileReadError(path.clone()))?,
        Ok(_) => markdown_to_html(markdown)
    }
}

pub fn markdown_to_html(markdown: String) -> Result<String> {
    match MarkdownParser::parse(Rule::markdown, &markdown) {
        Ok(mut parsed) => {
            let html = parse_markdown(parsed.next().unwrap());
            Ok(html)
        }
        Err(e) => Err(ParserError::MarkdownParseError(e.to_string()))?,
    }
}

fn parse_markdown(elem: pest::iterators::Pair<Rule>) -> String {
    let mut html = String::new();

    for pair in elem.into_inner() {
        match pair.as_rule() {
            Rule::header => html.push_str(&parse_header(pair)),
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
            Rule::header_hashtags => len = pair.as_str().len(),
            Rule::line_pars => val = parse_line_pars(pair),
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
            Rule::link_text => text = pair.as_str().to_owned(),
            Rule::link_href => href = pair.as_str().to_owned(),
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
