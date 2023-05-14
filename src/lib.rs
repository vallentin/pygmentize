//! Rust library and wrapper around the [pygmentize](https://pygments.org/docs/cmdline/) CLI.
//! Apply syntax highlighting to over 500 languages and other text formatted.
//! Render into HTML, SVG, LaTeX, and Terminal (ANSI color sequences).
//!
//! # Rendered HTML Output
//!
//! _Rendered example of [examples/html.rs](https://github.com/vallentin/pygmentize/blob/master/examples/html.rs)._
//!
//! ![Rendered Example](https://github.com/vallentin/pygmentize/assets/17464404/feedd372-9f36-4065-bbdd-0d7082ddbc0e)
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
//!
//! ## Rendered
//!
//! _(with the [Dracula theme](https://draculatheme.com))_
//!
//! ![image](https://user-images.githubusercontent.com/17464404/235512548-76086e98-dd04-4cff-90ab-e3cfde0d206c.png)
//!
//! ## Override Pygmentize Path
//!
//! The path to the `pygmentize` binary, can be overridden using `pygmentize::`[`set_bin_path()`](https://docs.rs/pygmentize/*/pygmentize/fn.set_bin_path.html). The default path is `"pygmentize"`.
//!
//! If `pygmentize` is installed in a virtual environment, within your crate directory,
//! i.e. `Cargo.lock` and `env/` being within the same directory. Then assuming that
//! the current directory is the same. Then the path can be overridden by doing:
//!
//! ```
//! pygmentize::set_bin_path("./env/Scripts/pygmentize");
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

use std::borrow::Cow;
use std::error;
use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Write};
use std::process::{Command, ExitStatus, Stdio};
use std::string::FromUtf8Error;
use std::sync::RwLock;

#[cfg(windows)]
use winapi_util::console::Console;

static PYGMENTIZE: RwLock<Cow<'static, str>> = RwLock::new(Cow::Borrowed("pygmentize"));

/// Overwrite the path to the `pygmentize` binary. The default path is `"pygmentize"`.
///
/// If `pygmentize` is installed in a virtual environment, within your crate directory,
/// i.e. `Cargo.lock` and `env/` being within the same directory. Then assuming that
/// the current directory is the same. Then the path can be overridden by doing:
///
/// ```no_run
/// pygmentize::set_bin_path("./env/Scripts/pygmentize");
/// ```
pub fn set_bin_path(pygmentize: impl Into<Cow<'static, str>>) {
    *PYGMENTIZE.write().unwrap() = pygmentize.into();
}

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
    let mut child = Command::new(PYGMENTIZE.read().unwrap().as_ref())
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
    ///
    /// The path to the `pygmentize` binary
    /// If `pygmentize` is installed but not added to
    /// the system PATH (e.g. if it is installed in a
    /// virtual environment), then it can manually be
    /// set by calling `pygmentize::`[`set_bin_path()`].
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
