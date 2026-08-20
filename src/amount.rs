use crate::error::ParseError;

/// A currency's shape: its ISO 4217 code, how many fractional digits its
/// minor unit has (2 for USD cents, 0 for JPY, 3 for BHD fils), and the
/// symbol used to write it informally, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Currency {
    pub code: &'static str,
    pub minor_units: u32,
    pub symbol: Option<char>,
}

// A starter set, not the full ISO 4217 list. BHD is included on purpose:
// three fractional digits is the case that breaks converters written with
// only USD/EUR-style currencies in mind.
pub const CURRENCIES: &[Currency] = &[
    Currency { code: "USD", minor_units: 2, symbol: Some('$') },
    Currency { code: "EUR", minor_units: 2, symbol: Some('€') },
    Currency { code: "GBP", minor_units: 2, symbol: Some('£') },
    Currency { code: "JPY", minor_units: 0, symbol: Some('¥') },
    Currency { code: "CHF", minor_units: 2, symbol: None },
    Currency { code: "BHD", minor_units: 3, symbol: None },
];

pub fn by_code(code: &str) -> Option<&'static Currency> {
    CURRENCIES.iter().find(|c| c.code.eq_ignore_ascii_case(code))
}

pub fn by_symbol(symbol: char) -> Option<&'static Currency> {
    CURRENCIES.iter().find(|c| c.symbol == Some(symbol))
}

/// An exact amount: a currency plus a signed integer count of its minor
/// units. This is the canonical, computation-safe representation that the
/// human-readable display format gets parsed into and formatted back from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Amount {
    pub currency: &'static Currency,
    pub minor_units: i64,
}

impl Amount {
    pub fn to_ledger(&self) -> String {
        format!("{} {}", self.currency.code, self.minor_units)
    }
}

/// Parse one line of the canonical ledger format: `<CODE> <integer>`,
/// e.g. `USD -4500` for -$45.00.
pub fn parse_ledger_line(line: &str, line_no: usize) -> Result<Amount, ParseError> {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }
    if i == chars.len() {
        return Err(ParseError::new(line_no, 1, "expected a currency code, found an empty line"));
    }

    let code_start = i;
    while i < chars.len() && !chars[i].is_whitespace() {
        i += 1;
    }
    let code: String = chars[code_start..i].iter().collect();
    let currency = by_code(&code).ok_or_else(|| {
        ParseError::new(line_no, code_start + 1, format!("unknown currency code '{}'", code))
    })?;

    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }
    if i == chars.len() {
        return Err(ParseError::new(
            line_no,
            i + 1,
            "expected an integer amount in minor units after the currency code",
        ));
    }

    let num_start = i;
    while i < chars.len() && !chars[i].is_whitespace() {
        i += 1;
    }
    let num_text: String = chars[num_start..i].iter().collect();

    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }
    if i != chars.len() {
        return Err(ParseError::new(line_no, i + 1, "unexpected trailing text after the amount"));
    }

    let minor_units: i64 = num_text.parse().map_err(|_| {
        ParseError::new(
            line_no,
            num_start + 1,
            format!("'{}' is not a valid integer number of minor units", num_text),
        )
    })?;

    Ok(Amount { currency, minor_units })
}
