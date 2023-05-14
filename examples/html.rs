use pygmentize::{highlight, HtmlFormatter};
use std::fs;

fn main() {
    let fmt = HtmlFormatter {
        line_numbers: true,
        ..HtmlFormatter::default()
    };

    let code = include_str!("html.rs");
    let html = highlight(code, Some("rust"), &fmt).unwrap();
    println!("{html}");

    // dracula.css is Dracula for Pygments
    // https://draculatheme.com/pygments
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en-US">
<head>
    <style>
        body {{
            color: #f8f8f2;
            background: #44475a;
        }}

        .highlight, .code {{
            background: #282a36;
        }}

        .linenos {{
            background: #44475a;
        }}
    </style>
    <link rel="stylesheet" href="dracula.css">
</head>
<body>
    {}
</body>
</html>"#,
        html
    );
    fs::write("index.html", html).unwrap();
}
