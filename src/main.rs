mod parser;
use anyhow::Result;
use parser::*;

fn main() -> Result<()> {
    let markdown = r#"
# Heading 1
## Heading 2

---

This is a paragraph with **bold text** and _italic text_.

- List item 1
- List item 2

1. List item 3
2. List item 4

[Link](https://example.com)

```
Code block content
print("Hello world!")
```
"#;
    let html = markdown_to_html(markdown.to_string())?;

    println!("{}", html);
    Ok(())
}
