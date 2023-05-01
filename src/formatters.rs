pub mod prelude {
    pub use super::{
        HtmlFormatter, LatexFormatter, PygmentizeFormatter, SvgFormatter, Terminal256Formatter,
        TerminalFormatter, TerminalTrueColorFormatter,
    };
}

use std::borrow::Cow;

use crate::{highlight, PygmentizeError};

/// Want to implement a formatter or add unsupported options?
///
/// See <https://pygments.org/docs/formatters/> for available
/// formatters.
pub trait PygmentizeFormatter: Sized {
    const SHORT_NAME: &'static str;

    fn options_str(&self) -> Option<Cow<'_, str>>;

    fn highlight(
        &self,
        code: impl AsRef<str>,
        lang: Option<&str>,
    ) -> Result<String, PygmentizeError> {
        highlight(code, lang, self)
    }
}

/// Format tokens as HTML 4 `<span>` tags.
///
/// See <https://pygments.org/docs/formatters/#HtmlFormatter>
/// for more information.
#[derive(Clone, Debug)]
pub struct HtmlFormatter {
    /// Output line numbers.
    pub line_numbers: bool,
}

impl Default for HtmlFormatter {
    fn default() -> Self {
        Self {
            line_numbers: false,
            // class_prefix: None,
        }
    }
}

impl HtmlFormatter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PygmentizeFormatter for HtmlFormatter {
    const SHORT_NAME: &'static str = "html";

    fn options_str(&self) -> Option<Cow<'_, str>> {
        if self.line_numbers {
            Some(Cow::Borrowed("linenos=true"))
        } else {
            None
        }
    }
}

/// Format tokens as an SVG graphics file. This formatter is still
/// experimental. Each line of code is a `<text>` element with explicit
/// x and y coordinates containing `<tspan>` elements with the
/// individual token styles.
///
/// See <https://pygments.org/docs/formatters/#SvgFormatter>
/// for more information.
#[derive(Clone, Debug)]
pub struct SvgFormatter {
    /// Output line numbers.
    pub line_numbers: bool,
}

impl Default for SvgFormatter {
    fn default() -> Self {
        Self {
            line_numbers: false,
        }
    }
}

impl SvgFormatter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PygmentizeFormatter for SvgFormatter {
    const SHORT_NAME: &'static str = "svg";

    fn options_str(&self) -> Option<Cow<'_, str>> {
        if self.line_numbers {
            Some(Cow::Borrowed("linenos=true"))
        } else {
            None
        }
    }
}

/// Format tokens as LaTeX code. This needs the `fancyvrb`
/// and `color` standard packages.
///
/// See <https://pygments.org/docs/formatters/#LatexFormatter>
/// for more information.
#[derive(Clone, Debug)]
pub struct LatexFormatter {
    /// Output line numbers.
    pub line_numbers: bool,
}

impl Default for LatexFormatter {
    fn default() -> Self {
        Self {
            line_numbers: false,
        }
    }
}

impl LatexFormatter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PygmentizeFormatter for LatexFormatter {
    const SHORT_NAME: &'static str = "latex";

    fn options_str(&self) -> Option<Cow<'_, str>> {
        if self.line_numbers {
            Some(Cow::Borrowed("linenos=true"))
        } else {
            None
        }
    }
}

/// Format tokens with ANSI color sequences, for output in a text
/// console. Color sequences are terminated at newlines, so that
/// paging the output works correctly.
///
/// See <https://pygments.org/docs/formatters/#TerminalFormatter>
/// for more information.
#[derive(Clone, Debug)]
pub struct TerminalFormatter {
    /// Output line numbers.
    pub line_numbers: bool,
}

impl Default for TerminalFormatter {
    fn default() -> Self {
        Self {
            line_numbers: false,
        }
    }
}

impl TerminalFormatter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PygmentizeFormatter for TerminalFormatter {
    const SHORT_NAME: &'static str = "terminal";

    fn options_str(&self) -> Option<Cow<'_, str>> {
        if self.line_numbers {
            Some(Cow::Borrowed("linenos=true"))
        } else {
            None
        }
    }
}

/// Format tokens with ANSI color sequences, for output in a true-color
/// terminal or console. Like in [`TerminalFormatter`] color sequences are
/// terminated at newlines, so that paging the output works correctly.
///
/// See <https://pygments.org/docs/formatters/#TerminalTrueColorFormatter>
/// for more information.
#[derive(Clone, Debug)]
pub struct TerminalTrueColorFormatter {
    /// Output line numbers.
    pub line_numbers: bool,
}

impl Default for TerminalTrueColorFormatter {
    fn default() -> Self {
        Self {
            line_numbers: false,
        }
    }
}

impl TerminalTrueColorFormatter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PygmentizeFormatter for TerminalTrueColorFormatter {
    const SHORT_NAME: &'static str = "terminal16m";

    fn options_str(&self) -> Option<Cow<'_, str>> {
        if self.line_numbers {
            Some(Cow::Borrowed("linenos=true"))
        } else {
            None
        }
    }
}

/// Format tokens with ANSI color sequences, for output in a 256-color
/// terminal or console. Like in [`TerminalFormatter`] color sequences are
/// terminated at newlines, so that paging the output works correctly.
///
/// See <https://pygments.org/docs/formatters/#Terminal256Formatter>
/// for more information.
#[derive(Clone, Debug)]
pub struct Terminal256Formatter {
    /// Output line numbers.
    pub line_numbers: bool,
}

impl Default for Terminal256Formatter {
    fn default() -> Self {
        Self {
            line_numbers: false,
        }
    }
}

impl Terminal256Formatter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PygmentizeFormatter for Terminal256Formatter {
    const SHORT_NAME: &'static str = "terminal256";

    fn options_str(&self) -> Option<Cow<'_, str>> {
        if self.line_numbers {
            Some(Cow::Borrowed("linenos=true"))
        } else {
            None
        }
    }
}
