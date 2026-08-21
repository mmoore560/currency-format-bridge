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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_amount() {
        let a = parse_ledger_line("USD 123456", 1).unwrap();
        assert_eq!(a.currency.code, "USD");
        assert_eq!(a.minor_units, 123456);
    }

    #[test]
    fn parses_negative_amount() {
        let a = parse_ledger_line("JPY -500", 1).unwrap();
        assert_eq!(a.currency.code, "JPY");
        assert_eq!(a.minor_units, -500);
    }

    #[test]
    fn code_lookup_is_case_insensitive() {
        let a = parse_ledger_line("usd 100", 1).unwrap();
        assert_eq!(a.currency.code, "USD");
    }

    #[test]
    fn ignores_leading_and_trailing_whitespace() {
        let a = parse_ledger_line("  USD 100  ", 1).unwrap();
        assert_eq!(a.minor_units, 100);
    }

    #[test]
    fn empty_line_reports_column_one() {
        let e = parse_ledger_line("", 7).unwrap_err();
        assert_eq!((e.line, e.column), (7, 1));
        assert!(e.message.contains("empty line"));
    }

    #[test]
    fn unknown_code_points_at_the_code() {
        let e = parse_ledger_line("XYZ 100", 3).unwrap_err();
        assert_eq!((e.line, e.column), (3, 1));
        assert!(e.message.contains("XYZ"));
    }

    #[test]
    fn unknown_code_column_accounts_for_leading_whitespace() {
        let e = parse_ledger_line("  XYZ 100", 1).unwrap_err();
        assert_eq!(e.column, 3);
    }

    #[test]
    fn missing_amount_after_code() {
        let e = parse_ledger_line("USD", 1).unwrap_err();
        assert_eq!(e.column, 4);
        assert!(e.message.contains("expected an integer amount"));
    }

    #[test]
    fn invalid_integer_points_at_the_number() {
        let e = parse_ledger_line("USD abc", 1).unwrap_err();
        assert_eq!(e.column, 5);
        assert!(e.message.contains("'abc'"));
    }

    #[test]
    fn trailing_text_after_amount() {
        let e = parse_ledger_line("USD 100 extra", 1).unwrap_err();
        assert_eq!(e.column, 9);
        assert!(e.message.contains("trailing text"));
    }
}
