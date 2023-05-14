# pygmentize

[![Latest Version](https://img.shields.io/crates/v/pygmentize.svg)](https://crates.io/crates/pygmentize)
[![Docs](https://docs.rs/pygmentize/badge.svg)](https://docs.rs/pygmentize)
![Pygmentize Version](https://img.shields.io/badge/pygmentize-2.15.1-blue)
[![License](https://img.shields.io/github/license/vallentin/pygmentize.svg)](https://github.com/vallentin/pygmentize)

Rust library and wrapper around the [pygmentize](https://pygments.org/docs/cmdline/) CLI. Apply syntax highlighting to over 500 languages and other text formatted. Render into HTML, SVG, LaTeX, and Terminal (ANSI color sequences).

## Rendered HTML Output

_Rendered example of [examples/html.rs](https://github.com/vallentin/pygmentize/blob/master/examples/html.rs)._

![Rendered Example](https://github.com/vallentin/pygmentize/assets/17464404/feedd372-9f36-4065-bbdd-0d7082ddbc0e)

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

![Rendered Example 2](https://user-images.githubusercontent.com/17464404/235512548-76086e98-dd04-4cff-90ab-e3cfde0d206c.png)

### Override Pygmentize Path

The path to the `pygmentize` binary, can be overridden using `pygmentize::`[`set_bin_path()`](https://docs.rs/pygmentize/*/pygmentize/fn.set_bin_path.html). The default path is `"pygmentize"`.

If `pygmentize` is installed in a virtual environment, within your crate directory,
i.e. `Cargo.lock` and `env/` being within the same directory. Then assuming that
the current directory is the same. Then the path can be overridden by doing:

```rust
pygmentize::set_bin_path("./env/Scripts/pygmentize");
```

## Install

The library is a wrapper around the [pygmentize](https://pygments.org/docs/cmdline/) CLI,
and as such it must be available in the system PATH. The easiest way to install
[pygmentize](https://pygments.org/docs/cmdline/) is through Python.

```console
pip install Pygments
```
