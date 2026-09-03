use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

mod amount;
mod display;
mod error;

use display::Locale;

enum Mode {
    ToLedger,
    ToDisplay,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut mode: Option<Mode> = None;
    let mut path: Option<String> = None;
    let mut locale = Locale::default();

    for arg in &args[1..] {
        match arg.as_str() {
            "--to-ledger" => mode = Some(Mode::ToLedger),
            "--to-display" => mode = Some(Mode::ToDisplay),
            other if other.starts_with("--locale=") => {
                let name = &other["--locale=".len()..];
                match Locale::parse(name) {
                    Some(l) => locale = l,
                    None => {
                        eprintln!("cfbridge: unknown locale '{}', expected 'us' or 'eu'", name);
                        return ExitCode::from(2);
                    }
                }
            }
            other => path = Some(other.to_string()),
        }
    }

    let mode = match mode {
        Some(m) => m,
        None => {
            eprintln!("usage: cfbridge --to-ledger|--to-display [--locale=us|eu] [file]");
            eprintln!("reads amounts, one per line, from the file, or from stdin if no file is given");
            eprintln!("--locale controls the display format's separators: us is '1,234.56', eu is '1.234,56' (default: us)");
            return ExitCode::from(2);
        }
    };

    let source = match &path {
        Some(p) => match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("cfbridge: could not read '{}': {}", p, e);
                return ExitCode::from(2);
            }
        },
        None => {
            let mut buf = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buf) {
                eprintln!("cfbridge: could not read stdin: {}", e);
                return ExitCode::from(2);
            }
            buf
        }
    };

    let mut had_error = false;
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if line.trim().is_empty() {
            continue;
        }
        let result = match mode {
            Mode::ToLedger => display::parse_display_line(line, line_no, locale).map(|a| a.to_ledger()),
            Mode::ToDisplay => {
                amount::parse_ledger_line(line, line_no).map(|a| display::format_display(&a, locale))
            }
        };
        match result {
            Ok(out) => println!("{}", out),
            Err(e) => {
                eprintln!("{}", e.render(&source));
                had_error = true;
            }
        }
    }

    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
