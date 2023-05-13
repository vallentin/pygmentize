//! Rust library and wrapper around the [pygmentize](https://pygments.org/docs/cmdline/) CLI.
//! Apply syntax highlighting to over 500 languages and other text formatted.
//! Render into HTML, SVG, LaTeX, and Terminal (ANSI color sequences).
//!
//! # Example
//!
//! ```rust
//! use pygmentize::{HtmlFormatter, PygmentizeError};
//!
//! # fn main() -> Result<(), PygmentizeError> {
//! let code = r#"fn main() {
//!     println!("Hello, world!");
//! }"#;
//!
//! let html = pygmentize::highlight(code, Some("rust"), &HtmlFormatter::default())?;
//! println!("{html}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Output
//!
//! _(whitespace added to improve clarity)_
//!
//! ```html
//! <div class="highlight">
//! <pre>
//!     <span></span>
//!
//!     <span class="k">fn</span>
//!     <span class="nf">main</span>
//!     <span class="p">()</span>
//!     <span class="w"> </span>
//!     <span class="p">{</span>
//!
//!     <span class="w">    </span>
//!     <span class="fm">println!</span>
//!     <span class="p">(</span>
//!     <span class="s">&quot;Hello, world!&quot;</span>
//!     <span class="p">);</span>
//!
//!     <span class="p">}</span>
//! </pre>
//! </div>
//! ```

#![deny(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_debug_implementations)]
#![cfg_attr(
    debug_assertions,
    allow(missing_docs, dead_code, unused_imports, unreachable_code)
)]

pub use formatters::prelude::*;

mod formatters;

use std::error;
use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Write};
use std::process::{Command, ExitStatus, Stdio};
use std::string::FromUtf8Error;

#[cfg(windows)]
use winapi_util::console::Console;

const PYGMENTIZE: &str = "pygmentize";

/// Applies syntax highlighting to `code` written in `lang`,
/// and outputs in the format of `F: `[`PygmentizeFormatter`].
///
/// If `lang` is `None` then the language is guessed from `code`.
/// Note though, that this option is not very reliable.
///
/// See supported languages at <https://pygments.org/languages/>.
///
/// # Example
///
/// ```rust
/// use pygmentize::{HtmlFormatter, PygmentizeError};
///
/// # fn main() -> Result<(), PygmentizeError> {
/// let code = r#"fn main() {
///     println!("Hello, world!");
/// }"#;
///
/// let html = pygmentize::highlight(code, Some("rust"), &HtmlFormatter::default())?;
/// println!("{html}");
/// # Ok(())
/// # }
/// ```
///
/// ## Output
///
/// _(whitespace added to improve clarity)_
///
/// ```html
/// <div class="highlight">
/// <pre>
///     <span></span>
///
///     <span class="k">fn</span>
///     <span class="nf">main</span>
///     <span class="p">()</span>
///     <span class="w"> </span>
///     <span class="p">{</span>
///
///     <span class="w">    </span>
///     <span class="fm">println!</span>
///     <span class="p">(</span>
///     <span class="s">&quot;Hello, world!&quot;</span>
///     <span class="p">);</span>
///
///     <span class="p">}</span>
/// </pre>
/// </div>
/// ```
pub fn highlight<F>(
    code: impl AsRef<str>,
    lang: Option<&str>,
    fmt: &F,
) -> Result<String, PygmentizeError>
where
    F: PygmentizeFormatter,
{
    let code = code.as_ref();
    let opt = fmt.options_str();
    let args = to_args(lang, F::SHORT_NAME, opt.as_deref());
    run_cmd(args, Some(code))
}

fn to_args<'a>(
    lang: Option<&'a str>,
    fmt_name: &'a str,
    options: Option<&'a str>,
) -> impl IntoIterator<Item = &'a str> + 'a {
    let mut args = [""; 6];
    args[0] = "-f";
    args[1] = fmt_name;
    let mut argi = 2;

    if let Some(lang) = lang {
        args[argi] = "-l";
        args[argi + 1] = lang;
        argi += 2;
    } else {
        args[argi] = "-g";
        argi += 1;
    }

    if let Some(opt) = options {
        args[argi] = "-O";
        args[argi + 1] = opt.as_ref();
        argi += 2;
    }

    debug_assert!(argi <= args.len());

    args.into_iter().filter(|arg| !arg.is_empty())
}

fn run_cmd<I, S>(args: I, stdin: Option<&str>) -> Result<String, PygmentizeError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new(PYGMENTIZE)
        .args(args)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| match err {
            _ if err.kind() == io::ErrorKind::NotFound => PygmentizeError::NotFound(err),
            _ => PygmentizeError::Process(err),
        })?;

    if let Some(data) = stdin {
        let mut stdin = child.stdin.take().expect("expected stdin");
        stdin
            .write_all(data.as_bytes())
            .map_err(PygmentizeError::Process)?;
        stdin.flush().map_err(PygmentizeError::Process)?;
        // Calling `wait_with_output()` closes stdin
    }

    let output = child.wait_with_output().map_err(PygmentizeError::Process)?;

    // Executing `pygmentize` causes `ENABLE_VIRTUAL_TERMINAL_PROCESSING` to get turned off
    #[cfg(windows)]
    enable_virtual_terminal_processing();

    if !output.status.success() {
        let stderr = match String::from_utf8(output.stderr) {
            Ok(stderr) => stderr,
            Err(err) => String::from_utf8_lossy(err.as_bytes()).into_owned(),
        };
        return Err(PygmentizeError::Pygmentize(output.status, stderr));
    }

    String::from_utf8(output.stdout).map_err(PygmentizeError::InvalidUtf8)
}

#[cfg(windows)]
fn enable_virtual_terminal_processing() {
    if let Ok(mut term) = Console::stdout() {
        #[allow(clippy::let_underscore_drop)]
        let _ = term.set_virtual_terminal_processing(true);
    }
    if let Ok(mut term) = Console::stderr() {
        #[allow(clippy::let_underscore_drop)]
        let _ = term.set_virtual_terminal_processing(true);
    }
}

#[derive(Debug)]
pub enum PygmentizeError {
    Process(io::Error),
    /// pygmentize was not found or not installed.
    NotFound(io::Error),
    InvalidUtf8(FromUtf8Error),
    /// The pygmentize binary returned an error.
    Pygmentize(ExitStatus, String),
}

impl error::Error for PygmentizeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Process(err) => Some(err),
            Self::NotFound(err) => Some(err),
            Self::InvalidUtf8(err) => Some(err),
            Self::Pygmentize(_, _) => None,
        }
    }
}

impl fmt::Display for PygmentizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Process(err) => err.fmt(f),
            Self::NotFound(_err) => {
                write!(f, "pygmentize was not found or not installed")
            }
            Self::InvalidUtf8(err) => err.fmt(f),
            Self::Pygmentize(status, stderr) => {
                write!(f, "pygmentize exited with {status}: {stderr}")
            }
        }
    }
}
