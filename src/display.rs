use crate::amount::{by_code, by_symbol, Amount};
use crate::error::ParseError;

/// Parse one line of the human-readable display format, e.g. `$1,234.56`
/// or `1.234,56` style inputs written the other way round as `1234.56 EUR`.
///
/// Grammar, roughly:
///   amount   := '-'? (symbol digits | digits ' ' code)
///   digits   := group (',' group3)* ('.' frac)?
///   group    := 1-3 digits (only constrained to <=3 when more groups follow)
///   group3   := exactly 3 digits
///   frac     := digits, count must match the currency's minor unit count
pub fn parse_display_line(line: &str, line_no: usize) -> Result<Amount, ParseError> {
    let chars: Vec<char> = line.chars().collect();

    let start = match chars.iter().position(|c| !c.is_whitespace()) {
        Some(s) => s,
        None => return Err(ParseError::new(line_no, 1, "line is empty")),
    };
    let end = chars.iter().rposition(|c| !c.is_whitespace()).unwrap();
    let body: Vec<char> = chars[start..=end].to_vec();
    // body indices are local to this trimmed slice; col_of maps them back to
    // a 1-based column in the original line.
    let col_of = |local: usize| start + local + 1;

    let mut i = 0;
    let negative = if body[i] == '-' {
        i += 1;
        true
    } else {
        false
    };

    let mut currency = None;
    if i < body.len() {
        if let Some(c) = by_symbol(body[i]) {
            currency = Some(c);
            i += 1;
        }
    }

    if i >= body.len() {
        return Err(ParseError::new(line_no, col_of(i), "expected a digit, found end of line"));
    }
    if !body[i].is_ascii_digit() {
        return Err(ParseError::new(
            line_no,
            col_of(i),
            format!("expected a digit, found '{}'", body[i]),
        ));
    }

    let int_start = i;
    let mut raw = String::new();
    while i < body.len() && (body[i].is_ascii_digit() || body[i] == ',') {
        raw.push(body[i]);
        i += 1;
    }
    if raw.ends_with(',') {
        return Err(ParseError::new(
            line_no,
            col_of(i - 1),
            "trailing ',' is not a valid thousands separator",
        ));
    }

    let groups: Vec<&str> = raw.split(',').collect();
    let mut int_digits = String::new();
    let mut group_col = int_start;
    for (gi, g) in groups.iter().enumerate() {
        if g.is_empty() {
            return Err(ParseError::new(
                line_no,
                col_of(group_col),
                "empty digit group next to a thousands separator",
            ));
        }
        if gi == 0 {
            if groups.len() > 1 && g.len() > 3 {
                return Err(ParseError::new(
                    line_no,
                    col_of(group_col),
                    format!(
                        "'{}' has {} digits before the first thousands separator, expected at most 3",
                        g,
                        g.len()
                    ),
                ));
            }
        } else if g.len() != 3 {
            return Err(ParseError::new(
                line_no,
                col_of(group_col),
                format!(
                    "'{}' has {} digits between thousands separators, expected exactly 3",
                    g,
                    g.len()
                ),
            ));
        }
        int_digits.push_str(g);
        group_col += g.len() + 1;
    }

    let mut frac_digits = String::new();
    let mut had_dot = false;
    let mut frac_col = col_of(i);
    if i < body.len() && body[i] == '.' {
        had_dot = true;
        let dot_idx = i;
        i += 1;
        frac_col = col_of(i);
        let frac_start = i;
        while i < body.len() && body[i].is_ascii_digit() {
            i += 1;
        }
        if i == frac_start {
            return Err(ParseError::new(
                line_no,
                col_of(dot_idx),
                "expected at least one digit after the decimal point",
            ));
        }
        frac_digits = body[frac_start..i].iter().collect();
    }

    let after_number = i;
    while i < body.len() && body[i] == ' ' {
        i += 1;
    }

    let currency = match currency {
        Some(c) => {
            if i != body.len() {
                return Err(ParseError::new(
                    line_no,
                    col_of(i),
                    "unexpected trailing text after a symbol-prefixed amount",
                ));
            }
            c
        }
        None => {
            if i == body.len() {
                return Err(ParseError::new(
                    line_no,
                    col_of(after_number),
                    "no currency symbol at the start and no currency code at the end; try '$12.34' or '12.34 USD'",
                ));
            }
            let code_start = i;
            while i < body.len() && body[i].is_ascii_alphabetic() {
                i += 1;
            }
            let code: String = body[code_start..i].iter().collect();
            if i != body.len() {
                return Err(ParseError::new(
                    line_no,
                    col_of(i),
                    "unexpected trailing text after the currency code",
                ));
            }
            by_code(&code).ok_or_else(|| {
                ParseError::new(line_no, col_of(code_start), format!("unknown currency code '{}'", code))
            })?
        }
    };

    if currency.minor_units == 0 && had_dot {
        return Err(ParseError::new(
            line_no,
            frac_col,
            format!("{} has no minor units; remove the decimal point", currency.code),
        ));
    }
    if had_dot && frac_digits.len() as u32 != currency.minor_units {
        return Err(ParseError::new(
            line_no,
            frac_col,
            format!(
                "{} amounts use {} fractional digit(s), but this amount has {}",
                currency.code,
                currency.minor_units,
                frac_digits.len()
            ),
        ));
    }

    let mut minor_str = int_digits;
    if currency.minor_units > 0 {
        if had_dot {
            minor_str.push_str(&frac_digits);
        } else {
            minor_str.extend(std::iter::repeat('0').take(currency.minor_units as usize));
        }
    }
    let magnitude: i64 = minor_str.parse().map_err(|_| {
        ParseError::new(line_no, col_of(int_start), "amount is too large to fit in a 64-bit integer")
    })?;

    Ok(Amount {
        currency,
        minor_units: if negative { -magnitude } else { magnitude },
    })
}

/// Format an `Amount` back into the human-readable display format, using the
/// currency's symbol when it has one and a trailing ISO code otherwise.
pub fn format_display(amount: &Amount) -> String {
    let c = amount.currency;
    let negative = amount.minor_units < 0;
    let magnitude = amount.minor_units.unsigned_abs();
    let digits = magnitude.to_string();

    let (int_part, frac_part) = if c.minor_units == 0 {
        (digits, String::new())
    } else {
        let mu = c.minor_units as usize;
        let padded = if digits.len() <= mu {
            format!("{:0>width$}", digits, width = mu + 1)
        } else {
            digits
        };
        let split_at = padded.len() - mu;
        (padded[..split_at].to_string(), padded[split_at..].to_string())
    };

    let grouped_int = group_thousands(&int_part);

    let mut out = String::new();
    if negative {
        out.push('-');
    }
    match c.symbol {
        Some(sym) => {
            out.push(sym);
            out.push_str(&grouped_int);
            if !frac_part.is_empty() {
                out.push('.');
                out.push_str(&frac_part);
            }
        }
        None => {
            out.push_str(&grouped_int);
            if !frac_part.is_empty() {
                out.push('.');
                out.push_str(&frac_part);
            }
            out.push(' ');
            out.push_str(c.code);
        }
    }
    out
}

fn group_thousands(digits: &str) -> String {
    let bytes = digits.as_bytes();
    let len = bytes.len();
    let mut out = Vec::with_capacity(len + len / 3);
    for (idx, b) in bytes.iter().enumerate() {
        if idx > 0 && (len - idx) % 3 == 0 {
            out.push(b',');
        }
        out.push(*b);
    }
    String::from_utf8(out).unwrap()
}
