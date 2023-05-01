# pygmentize

[![Latest Version](https://img.shields.io/crates/v/pygmentize.svg)](https://crates.io/crates/pygmentize)
[![Docs](https://docs.rs/pygmentize/badge.svg)](https://docs.rs/pygmentize)
![Pygmentize Version](https://img.shields.io/badge/pygmentize-2.15.1-blue)
[![License](https://img.shields.io/github/license/vallentin/pygmentize.svg)](https://github.com/vallentin/pygmentize)

Rust library and wrapper around the [pygmentize](https://pygments.org/docs/cmdline/) CLI. Apply syntax highlighting to over 500 languages and other text formatted. Render into HTML, SVG, LaTeX, and Terminal (ANSI color sequences).

## Example

```rust
use pygmentize::{HtmlFormatter, PygmentizeError};

let code = r#"fn main() {
    println!("Hello, world!");
}"#;

let html = pygmentize::highlight(code, Some("rust"), &HtmlFormatter::default())?;
println!("{html}");
```

### Output

_(whitespace added to improve clarity)_

```html
<div class="highlight">
<pre>
    <span></span>

    <span class="k">fn</span>
    <span class="nf">main</span>
    <span class="p">()</span>
    <span class="w"> </span>
    <span class="p">{</span>

    <span class="w">    </span>
    <span class="fm">println!</span>
    <span class="p">(</span>
    <span class="s">&quot;Hello, world!&quot;</span>
    <span class="p">);</span>

    <span class="p">}</span>
</pre>
</div>
```

### Rendered

_(with the [Dracula theme](https://draculatheme.com))_

![image](https://user-images.githubusercontent.com/17464404/235512548-76086e98-dd04-4cff-90ab-e3cfde0d206c.png)

## Install

The library is a wrapper around the [pygmentize](https://pygments.org/docs/cmdline/) CLI,
and as such it must be available in the system PATH. The easiest way to install
[pygmentize](https://pygments.org/docs/cmdline/) is through Python.

```console
pip install Pygments
```
