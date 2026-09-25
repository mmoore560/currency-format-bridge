use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

mod amount;
mod display;
mod error;
mod totals;

use display::Locale;
use totals::Totals;

enum Mode {
    ToLedger,
    ToDisplay,
    Validate,
    Totals,
}

/// Which format `--totals` reads its input lines as. Every other mode's
/// input format is implied by the mode itself (`--to-ledger` reads display,
/// `--to-display` reads ledger), but totals can sensibly sum either one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputFormat {
    Display,
    Ledger,
}

impl InputFormat {
    fn parse(name: &str) -> Option<InputFormat> {
        match name {
            "display" => Some(InputFormat::Display),
            "ledger" => Some(InputFormat::Ledger),
            _ => None,
        }
    }
}

impl Default for InputFormat {
    fn default() -> Self {
        InputFormat::Display
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut mode: Option<Mode> = None;
    let mut path: Option<String> = None;
    let mut locale = Locale::default();
    let mut input_format = InputFormat::default();

    for arg in &args[1..] {
        match arg.as_str() {
            "--to-ledger" => mode = Some(Mode::ToLedger),
            "--to-display" => mode = Some(Mode::ToDisplay),
            "--validate" => mode = Some(Mode::Validate),
            "--totals" => mode = Some(Mode::Totals),
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
            other if other.starts_with("--input=") => {
                let name = &other["--input=".len()..];
                match InputFormat::parse(name) {
                    Some(f) => input_format = f,
                    None => {
                        eprintln!("cfbridge: unknown input format '{}', expected 'display' or 'ledger'", name);
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
            eprintln!(
                "usage: cfbridge --to-ledger|--to-display|--validate|--totals [--locale=us|eu] [--input=display|ledger] [file]"
            );
            eprintln!("reads amounts, one per line, from the file, or from stdin if no file is given");
            eprintln!("--locale controls the display format's separators: us is '1,234.56', eu is '1.234,56' (default: us)");
            eprintln!("--validate reads display-format lines and reports any that are not in canonical form");
            eprintln!("--totals prints one summed line per currency code");
            eprintln!("--input selects the format --totals reads, display or ledger (default: display)");
            return ExitCode::from(2);
        }
    };

    if input_format == InputFormat::Ledger && !matches!(mode, Mode::Totals) {
        eprintln!("cfbridge: --input=ledger only applies to --totals");
        return ExitCode::from(2);
    }

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
    let mut totals = Totals::new();
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if line.trim().is_empty() {
            continue;
        }
        if let Mode::Validate = mode {
            match display::round_trip(line, line_no, locale) {
                Ok(canonical) if canonical == line.trim() => println!("{}: ok", line_no),
                Ok(canonical) => {
                    println!("{}: not canonical, canonical form is '{}'", line_no, canonical);
                    had_error = true;
                }
                Err(e) => {
                    eprintln!("{}", e.render(&source));
                    had_error = true;
                }
            }
            continue;
        }
        if let Mode::Totals = mode {
            let parsed = match input_format {
                InputFormat::Display => display::parse_display_line(line, line_no, locale),
                InputFormat::Ledger => amount::parse_ledger_line(line, line_no),
            };
            match parsed {
                Ok(amount) => {
                    if totals.add(&amount).is_err() {
                        let err = error::ParseError::new(
                            line_no,
                            1,
                            format!("running total for {} overflowed a 64-bit integer", amount.currency.code),
                        );
                        eprintln!("{}", err.render(&source));
                        had_error = true;
                    }
                }
                Err(e) => {
                    eprintln!("{}", e.render(&source));
                    had_error = true;
                }
            }
            continue;
        }

        let result = match mode {
            Mode::ToLedger => display::parse_display_line(line, line_no, locale).map(|a| a.to_ledger()),
            Mode::ToDisplay => {
                amount::parse_ledger_line(line, line_no).map(|a| display::format_display(&a, locale))
            }
            Mode::Validate | Mode::Totals => unreachable!("handled above"),
        };
        match result {
            Ok(out) => println!("{}", out),
            Err(e) => {
                eprintln!("{}", e.render(&source));
                had_error = true;
            }
        }
    }

    if let Mode::Totals = mode {
        for (code, minor_units) in totals.iter() {
            let amount = amount::Amount { currency: amount::by_code(code).unwrap(), minor_units };
            println!("{}", display::format_display(&amount, locale));
        }
    }

    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
