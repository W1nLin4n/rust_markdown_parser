#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pest::Parser;
    use pest_derive::Parser;
    use rust_markdown_parser::markdown_file_to_html;
    use rust_markdown_parser::markdown_to_html;

    #[derive(Parser)]
    #[grammar = "markdown.pest"]
    struct MarkdownParser;

    fn parse_success(input: &str, rule: Rule) {
        match MarkdownParser::parse(rule, input) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Failed to parse input:\n{}\nError: {}", input, e),
        }
    }

    fn parse_fail(input: &str, rule: Rule) {
        match MarkdownParser::parse(rule, input) {
            Ok(_) => panic!("Input should not parse successfully:\n{}", input),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_space_parsing() -> Result<()> {
        let inputs = [" ", "\t"];
        for input in inputs {
            parse_success(input, Rule::space);
        }

        parse_fail("\n", Rule::space);
        Ok(())
    }

    #[test]
    fn test_newline_parsing() -> Result<()> {
        let inputs = ["\n", "\r\n"];
        for input in inputs {
            parse_success(input, Rule::newline);
        }

        parse_fail("  ", Rule::newline);
        Ok(())
    }

    #[test]
    fn test_blankline_parsing() -> Result<()> {
        let inputs = ["\n", "\n\r\n\n", "   \t  \n\t   \t\r\n   \t\n"];
        for input in inputs {
            parse_success(input, Rule::blankline);
        }

        parse_fail("abc\n", Rule::newline);
        Ok(())
    }

    #[test]
    fn test_block_parsing() -> Result<()> {
        let inputs = ["abc\ndef", "# Hello world", "1. Hello\n2. World"];
        for input in inputs {
            parse_success(input, Rule::block);
        }
        Ok(())
    }

    #[test]
    fn test_special_block_parsing() -> Result<()> {
        let inputs = ["# Hello world", "1. Hello\n2. World"];
        for input in inputs {
            parse_success(input, Rule::special_block);
        }

        parse_fail("abc\ndef", Rule::special_block);
        Ok(())
    }

    #[test]
    fn test_header_parsing() -> Result<()> {
        let inputs = [
            "# Header 1",
            "## Header 2",
            "### Header 3",
            "###### Header 6",
        ];
        for input in inputs {
            parse_success(input, Rule::header);
        }

        parse_fail("#InvalidHeader", Rule::header);
        parse_fail("####### Too many hashtags", Rule::header);
        Ok(())
    }

    #[test]
    fn test_header_hashtags_parsing() -> Result<()> {
        let inputs = ["#", "##", "###", "######"];
        for input in inputs {
            parse_success(input, Rule::header_hashtags);
        }

        parse_fail("abc", Rule::header_hashtags);
        Ok(())
    }

    #[test]
    fn test_thematic_break_parsing() -> Result<()> {
        let inputs = ["---", "*******", "___"];
        for input in inputs {
            parse_success(input, Rule::thematic_break);
        }

        parse_fail("--", Rule::thematic_break);
        parse_fail("++++", Rule::thematic_break);
        Ok(())
    }

    #[test]
    fn test_list_parsing() -> Result<()> {
        let inputs = [
            "- Item 1\n- Item 2",
            "1. Item A\n2. Item B",
            "+ Item X\n+ Item Y",
        ];
        for input in inputs {
            parse_success(input, Rule::list);
        }

        parse_fail("1 ItemNoSpace", Rule::list);
        parse_fail("+Item", Rule::list);
        Ok(())
    }

    #[test]
    fn test_unordered_list_parsing() -> Result<()> {
        let inputs = [
            "- Item 1\n- Item 2",
            "* Item A\n* Item B",
            "+ Item X\n+ Item Y",
        ];
        for input in inputs {
            parse_success(input, Rule::unordered_list);
        }

        parse_fail("-ItemNoSpace", Rule::unordered_list);
        parse_fail("+Item", Rule::unordered_list);
        Ok(())
    }

    #[test]
    fn test_ordered_list_parsing() -> Result<()> {
        let inputs = ["1. Item 1\n2. Item 2", "10. Item A\n11. Item B"];
        for input in inputs {
            parse_success(input, Rule::ordered_list);
        }

        parse_fail("1 ItemMissingPeriod", Rule::ordered_list);
        Ok(())
    }

    #[test]
    fn test_code_block_parsing() -> Result<()> {
        let inputs = [
            "```\nCode block content\n```",
            "```\nMultiline\nCode block\n```",
        ];
        for input in inputs {
            parse_success(input, Rule::code_block);
        }

        parse_fail("```\nUnclosed code block", Rule::code_block);
        parse_fail("``` Code without newline", Rule::code_block);
        Ok(())
    }

    #[test]
    fn test_paragraph_parsing() -> Result<()> {
        let inputs = [
            "This is a simple paragraph.",
            "A paragraph with multiple lines\nspanning two or more lines.",
        ];
        for input in inputs {
            parse_success(input, Rule::paragraph);
        }
        Ok(())
    }

    #[test]
    fn test_line_parsing() -> Result<()> {
        let inputs = [
            "This is a [link](https://example.com).",
            "Some **bold text** here.",
            "An _italic word_.",
            "Combined **bold and _italic_** formatting.",
        ];
        for input in inputs {
            parse_success(input, Rule::line);
        }
        Ok(())
    }

    #[test]
    fn test_line_pars_parsing() -> Result<()> {
        let inputs = [
            "This is a [link](https://example.com).",
            "Some **bold text** here.",
            "An _italic word_.",
            "Combined **bold and _italic_** formatting.",
        ];
        for input in inputs {
            parse_success(input, Rule::line_pars);
        }
        parse_fail("\nHello World", Rule::line_pars);
        Ok(())
    }

    #[test]
    fn test_inline_parsing() -> Result<()> {
        let inputs = [
            "This is a [link](https://example.com).",
            "Some **bold text** here.",
            "An _italic word_.",
            "Combined **bold and _italic_** formatting.",
        ];
        for input in inputs {
            parse_success(input, Rule::line_pars);
        }
        parse_fail("\nHello World", Rule::inline);
        Ok(())
    }

    #[test]
    fn test_link_parsing() -> Result<()> {
        let inputs = ["[link](https://example.com)", "[]()"];
        for input in inputs {
            parse_success(input, Rule::link);
        }
        parse_fail("[link]https://example.com", Rule::link);
        parse_fail("link(https://example.com)", Rule::link);
        Ok(())
    }

    #[test]
    fn test_link_text_parsing() -> Result<()> {
        let inputs = ["asd]", "asd"];
        for input in inputs {
            parse_success(input, Rule::link_text);
        }
        Ok(())
    }

    #[test]
    fn test_link_href_parsing() -> Result<()> {
        let inputs = ["asd)", "asd"];
        for input in inputs {
            parse_success(input, Rule::link_href);
        }
        Ok(())
    }

    #[test]
    fn test_bold_parsing() -> Result<()> {
        let inputs = ["**bold**", "__bold__"];
        for input in inputs {
            parse_success(input, Rule::bold);
        }
        parse_fail("**bold", Rule::bold);
        parse_fail("bold__", Rule::bold);
        Ok(())
    }

    #[test]
    fn test_italic_parsing() -> Result<()> {
        let inputs = ["*italic*", "_italic_"];
        for input in inputs {
            parse_success(input, Rule::italic);
        }
        parse_fail("*italic", Rule::italic);
        parse_fail("italic_", Rule::italic);
        Ok(())
    }

    #[test]
    fn test_text_parsing() -> Result<()> {
        let inputs = ["hello", "even multi word"];
        for input in inputs {
            parse_success(input, Rule::text);
        }
        parse_fail("\nabc", Rule::text);
        Ok(())
    }

    #[test]
    fn test_markdown_parsing() -> Result<()> {
        let html1 = markdown_to_html(String::from(
            r#"# Heading 1
## Heading 2

---

This is a paragraph with **bold text** and _italic text_.

- List item 1
- List item 2

1. List item 3
2. List item 4

[Link](https://example.com)

```
# Code block content
print("Hello world!")
```
"#,
        ))?;
        let html2 = markdown_to_html(String::from(
            r#"# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Hello, 
world

--------
***********
_____

- List **item 1**
- List item 2
+ List item 3
+ List __item 4__
* List *item 5*
* List item 6
1. List _item 7_
2. List item 8

[Link](https://example.com)
```
# Code block content
print("Hello world!")
# Good code
```
"#,
        ))?;
        assert_eq!(
            html1,
            r#"<h1>Heading 1</h1>
<h2>Heading 2</h2>
<hr/>
<p>This is a paragraph with <strong>bold text</strong> and <em>italic text</em>.</p>
<ul>
<li>List item 1</li>
<li>List item 2</li>
</ul>
<ol>
<li>List item 3</li>
<li>List item 4</li>
</ol>
<p><a href="https://example.com">Link</a></p>
<pre><code>
# Code block content
print("Hello world!")
</code></pre>
"#
        );
        assert_eq!(
            html2,
            r#"<h1>Heading 1</h1>
<h2>Heading 2</h2>
<h3>Heading 3</h3>
<h4>Heading 4</h4>
<h5>Heading 5</h5>
<h6>Heading 6</h6>
<p>Hello, </p>
<p>world</p>
<hr/>
<hr/>
<hr/>
<ul>
<li>List <strong>item 1</strong></li>
<li>List item 2</li>
<li>List item 3</li>
<li>List <strong>item 4</strong></li>
<li>List <em>item 5</em></li>
<li>List item 6</li>
</ul>
<ol>
<li>List <em>item 7</em></li>
<li>List item 8</li>
</ol>
<p><a href="https://example.com">Link</a></p>
<pre><code>
# Code block content
print("Hello world!")
# Good code
</code></pre>
"#
        );
        Ok(())
    }

    #[test]
    fn test_markdown_file_parsing() -> Result<()> {
        let html1 = markdown_file_to_html(String::from("samples/A.md"))?;
        let html2 = markdown_file_to_html(String::from("samples/B.md"))?;
        assert_eq!(
            html1,
            r#"<h1>Heading 1</h1>
<h2>Heading 2</h2>
<hr/>
<p>This is a paragraph with <strong>bold text</strong> and <em>italic text</em>.</p>
<ul>
<li>List item 1</li>
<li>List item 2</li>
</ul>
<ol>
<li>List item 3</li>
<li>List item 4</li>
</ol>
<p><a href="https://example.com">Link</a></p>
<pre><code>
# Code block content
print("Hello world!")
</code></pre>
"#
        );
        assert_eq!(
            html2,
            r#"<h1>Heading 1</h1>
<h2>Heading 2</h2>
<h3>Heading 3</h3>
<h4>Heading 4</h4>
<h5>Heading 5</h5>
<h6>Heading 6</h6>
<p>Hello, </p>
<p>world</p>
<hr/>
<hr/>
<hr/>
<ul>
<li>List <strong>item 1</strong></li>
<li>List item 2</li>
<li>List item 3</li>
<li>List <strong>item 4</strong></li>
<li>List <em>item 5</em></li>
<li>List item 6</li>
</ul>
<ol>
<li>List <em>item 7</em></li>
<li>List item 8</li>
</ol>
<p><a href="https://example.com">Link</a></p>
<pre><code>
# Code block content
print("Hello world!")
# Good code
</code></pre>
"#
        );
        Ok(())
    }
}
