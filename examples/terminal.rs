use pygmentize::{highlight, Terminal256Formatter};

fn main() {
    // See also `TerminalFormatter` and `TerminalTrueColorFormatter`
    let fmt = Terminal256Formatter {
        line_numbers: false,
        ..Terminal256Formatter::default()
    };

    let code = include_str!("terminal.rs");
    let output = highlight(code, Some("rust"), &fmt).unwrap();
    println!("{output}");
}
