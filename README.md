# rust_markdown_parser
Simplified markdown parser implemented in Rust. Converts markdown into HTML.

## Features
This implementation can handle these markdown features:
- Headers
- Thematic breaks
- Lists:
  - Unordered lists
  - Ordered lists
- Code blocks
- Paragraphs
- Links
- Bold and italic text styles

## Limitations
For features described above to work, some limitations are applied:
- Text indentaion is not supported. Any line that starts with a space or a tab
will be converted to `<p>` element;
- Nested lists are not supported;
- Nesting of elements in general is not well supported;

## Implementation details
### Parsing process
The library used for parsing in the project is pest. It is used to build a DAG based on markdown string, and later this DAG is used in construction of an HTML string. To build a DAG it uses grammar, provided in `markdown.pest`.
  
Talking about parsing logic - there are two main steps:
1. Parsing all input into block elements.
2. Parsing inline elements of each block.  

As we can see, looking at the grammar file, **block** rule is responsible for the first step, while **line** and **line_pars** are responsible for the second step.

### Grammar rules
1. **space**: whitespaces
2. **newline**: new line symbols
3. **blankline**: one or more lines that are empty(only spaces allowed)
4. **markdown**: whole file structure
5. **block**: block element
6. **special_block**: all blocks except paragraph
7. **header**: header element
8. **header_hashtags**: used to select header variant
9. **thematic_break**: thematic break element
10. **list**: combined rule for both lists' variants
11. **unordered_list**: unordered list element
12. **ordered_list**: ordered list element
13. **code_block**: code block element
14. **paragraph**: paragraph element
15. **line**: captures all elements to the end of line
16. **line_pars**: same as line, but also parses it into inline elements
17. **inline**: inline element
18. **link** = link element
19. **link_text** = captures link text
20. **link_href** = captures link href
21. **bold** = bold element
22. **italic** = italic element
23. **text** = works like line, but stops when other inline elements can be captured