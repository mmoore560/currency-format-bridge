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

// Every currency in active circulation per ISO 4217, covering all three
// fractional-digit counts the standard uses (0, 2, and 3). Fund codes and
// precious-metal codes (BOV, XAU, XDR, ...) are left out since they never
// show up in a display-format amount. Only the four currencies with a
// widely recognized single-character symbol get one; everything else is
// written with a trailing code (`12.34 CAD`), same as CHF and BHD below.
pub const CURRENCIES: &[Currency] = &[
    Currency { code: "USD", minor_units: 2, symbol: Some('$') },
    Currency { code: "EUR", minor_units: 2, symbol: Some('€') },
    Currency { code: "GBP", minor_units: 2, symbol: Some('£') },
    Currency { code: "JPY", minor_units: 0, symbol: Some('¥') },
    Currency { code: "CHF", minor_units: 2, symbol: None },
    Currency { code: "BHD", minor_units: 3, symbol: None },
    // Zero-decimal currencies (ISO 4217 defines no minor unit).
    Currency { code: "BIF", minor_units: 0, symbol: None },
    Currency { code: "CLP", minor_units: 0, symbol: None },
    Currency { code: "DJF", minor_units: 0, symbol: None },
    Currency { code: "GNF", minor_units: 0, symbol: None },
    Currency { code: "ISK", minor_units: 0, symbol: None },
    Currency { code: "KMF", minor_units: 0, symbol: None },
    Currency { code: "KRW", minor_units: 0, symbol: None },
    Currency { code: "PYG", minor_units: 0, symbol: None },
    Currency { code: "RWF", minor_units: 0, symbol: None },
    Currency { code: "UGX", minor_units: 0, symbol: None },
    Currency { code: "VND", minor_units: 0, symbol: None },
    Currency { code: "VUV", minor_units: 0, symbol: None },
    Currency { code: "XAF", minor_units: 0, symbol: None },
    Currency { code: "XOF", minor_units: 0, symbol: None },
    Currency { code: "XPF", minor_units: 0, symbol: None },
    // Three-decimal currencies, the same family BHD belongs to.
    Currency { code: "IQD", minor_units: 3, symbol: None },
    Currency { code: "JOD", minor_units: 3, symbol: None },
    Currency { code: "KWD", minor_units: 3, symbol: None },
    Currency { code: "LYD", minor_units: 3, symbol: None },
    Currency { code: "OMR", minor_units: 3, symbol: None },
    Currency { code: "TND", minor_units: 3, symbol: None },
    // Two-decimal currencies (the common case).
    Currency { code: "AED", minor_units: 2, symbol: None },
    Currency { code: "ARS", minor_units: 2, symbol: None },
    Currency { code: "AUD", minor_units: 2, symbol: None },
    Currency { code: "BDT", minor_units: 2, symbol: None },
    Currency { code: "BGN", minor_units: 2, symbol: None },
    Currency { code: "BRL", minor_units: 2, symbol: None },
    Currency { code: "CAD", minor_units: 2, symbol: None },
    Currency { code: "CNY", minor_units: 2, symbol: None },
    Currency { code: "COP", minor_units: 2, symbol: None },
    Currency { code: "CZK", minor_units: 2, symbol: None },
    Currency { code: "DKK", minor_units: 2, symbol: None },
    Currency { code: "EGP", minor_units: 2, symbol: None },
    Currency { code: "HKD", minor_units: 2, symbol: None },
    Currency { code: "HUF", minor_units: 2, symbol: None },
    Currency { code: "IDR", minor_units: 2, symbol: None },
    Currency { code: "ILS", minor_units: 2, symbol: None },
    Currency { code: "INR", minor_units: 2, symbol: None },
    Currency { code: "KES", minor_units: 2, symbol: None },
    Currency { code: "KZT", minor_units: 2, symbol: None },
    Currency { code: "LKR", minor_units: 2, symbol: None },
    Currency { code: "MAD", minor_units: 2, symbol: None },
    Currency { code: "MXN", minor_units: 2, symbol: None },
    Currency { code: "MYR", minor_units: 2, symbol: None },
    Currency { code: "NGN", minor_units: 2, symbol: None },
    Currency { code: "NOK", minor_units: 2, symbol: None },
    Currency { code: "NPR", minor_units: 2, symbol: None },
    Currency { code: "NZD", minor_units: 2, symbol: None },
    Currency { code: "PEN", minor_units: 2, symbol: None },
    Currency { code: "PHP", minor_units: 2, symbol: None },
    Currency { code: "PKR", minor_units: 2, symbol: None },
    Currency { code: "PLN", minor_units: 2, symbol: None },
    Currency { code: "QAR", minor_units: 2, symbol: None },
    Currency { code: "RON", minor_units: 2, symbol: None },
    Currency { code: "RSD", minor_units: 2, symbol: None },
    Currency { code: "RUB", minor_units: 2, symbol: None },
    Currency { code: "SAR", minor_units: 2, symbol: None },
    Currency { code: "SEK", minor_units: 2, symbol: None },
    Currency { code: "SGD", minor_units: 2, symbol: None },
    Currency { code: "THB", minor_units: 2, symbol: None },
    Currency { code: "TRY", minor_units: 2, symbol: None },
    Currency { code: "TWD", minor_units: 2, symbol: None },
    Currency { code: "UAH", minor_units: 2, symbol: None },
    Currency { code: "UYU", minor_units: 2, symbol: None },
    Currency { code: "VES", minor_units: 2, symbol: None },
    Currency { code: "ZAR", minor_units: 2, symbol: None },
    // The rest of the actively circulating ISO 4217 currencies, all
    // two-decimal. Fund codes (BOV, CHE, USN, ...) and metal codes (XAU,
    // XAG, XDR, ...) are left out since nobody writes a grocery receipt
    // in them.
    Currency { code: "AFN", minor_units: 2, symbol: None },
    Currency { code: "ALL", minor_units: 2, symbol: None },
    Currency { code: "AMD", minor_units: 2, symbol: None },
    Currency { code: "ANG", minor_units: 2, symbol: None },
    Currency { code: "AOA", minor_units: 2, symbol: None },
    Currency { code: "AWG", minor_units: 2, symbol: None },
    Currency { code: "AZN", minor_units: 2, symbol: None },
    Currency { code: "BAM", minor_units: 2, symbol: None },
    Currency { code: "BBD", minor_units: 2, symbol: None },
    Currency { code: "BMD", minor_units: 2, symbol: None },
    Currency { code: "BND", minor_units: 2, symbol: None },
    Currency { code: "BOB", minor_units: 2, symbol: None },
    Currency { code: "BSD", minor_units: 2, symbol: None },
    Currency { code: "BTN", minor_units: 2, symbol: None },
    Currency { code: "BWP", minor_units: 2, symbol: None },
    Currency { code: "BYN", minor_units: 2, symbol: None },
    Currency { code: "BZD", minor_units: 2, symbol: None },
    Currency { code: "CDF", minor_units: 2, symbol: None },
    Currency { code: "CRC", minor_units: 2, symbol: None },
    Currency { code: "CUP", minor_units: 2, symbol: None },
    Currency { code: "CVE", minor_units: 2, symbol: None },
    Currency { code: "DOP", minor_units: 2, symbol: None },
    Currency { code: "DZD", minor_units: 2, symbol: None },
    Currency { code: "ERN", minor_units: 2, symbol: None },
    Currency { code: "ETB", minor_units: 2, symbol: None },
    Currency { code: "FJD", minor_units: 2, symbol: None },
    Currency { code: "FKP", minor_units: 2, symbol: None },
    Currency { code: "GEL", minor_units: 2, symbol: None },
    Currency { code: "GHS", minor_units: 2, symbol: None },
    Currency { code: "GIP", minor_units: 2, symbol: None },
    Currency { code: "GMD", minor_units: 2, symbol: None },
    Currency { code: "GTQ", minor_units: 2, symbol: None },
    Currency { code: "GYD", minor_units: 2, symbol: None },
    Currency { code: "HNL", minor_units: 2, symbol: None },
    Currency { code: "HTG", minor_units: 2, symbol: None },
    Currency { code: "IRR", minor_units: 2, symbol: None },
    Currency { code: "JMD", minor_units: 2, symbol: None },
    Currency { code: "KGS", minor_units: 2, symbol: None },
    Currency { code: "KHR", minor_units: 2, symbol: None },
    Currency { code: "KPW", minor_units: 2, symbol: None },
    Currency { code: "KYD", minor_units: 2, symbol: None },
    Currency { code: "LAK", minor_units: 2, symbol: None },
    Currency { code: "LBP", minor_units: 2, symbol: None },
    Currency { code: "LRD", minor_units: 2, symbol: None },
    Currency { code: "LSL", minor_units: 2, symbol: None },
    Currency { code: "MDL", minor_units: 2, symbol: None },
    Currency { code: "MGA", minor_units: 2, symbol: None },
    Currency { code: "MKD", minor_units: 2, symbol: None },
    Currency { code: "MMK", minor_units: 2, symbol: None },
    Currency { code: "MNT", minor_units: 2, symbol: None },
    Currency { code: "MOP", minor_units: 2, symbol: None },
    Currency { code: "MRU", minor_units: 2, symbol: None },
    Currency { code: "MUR", minor_units: 2, symbol: None },
    Currency { code: "MVR", minor_units: 2, symbol: None },
    Currency { code: "MWK", minor_units: 2, symbol: None },
    Currency { code: "MZN", minor_units: 2, symbol: None },
    Currency { code: "NAD", minor_units: 2, symbol: None },
    Currency { code: "NIO", minor_units: 2, symbol: None },
    Currency { code: "PAB", minor_units: 2, symbol: None },
    Currency { code: "PGK", minor_units: 2, symbol: None },
    Currency { code: "SBD", minor_units: 2, symbol: None },
    Currency { code: "SCR", minor_units: 2, symbol: None },
    Currency { code: "SDG", minor_units: 2, symbol: None },
    Currency { code: "SHP", minor_units: 2, symbol: None },
    Currency { code: "SLE", minor_units: 2, symbol: None },
    Currency { code: "SOS", minor_units: 2, symbol: None },
    Currency { code: "SRD", minor_units: 2, symbol: None },
    Currency { code: "SSP", minor_units: 2, symbol: None },
    Currency { code: "STN", minor_units: 2, symbol: None },
    Currency { code: "SVC", minor_units: 2, symbol: None },
    Currency { code: "SYP", minor_units: 2, symbol: None },
    Currency { code: "SZL", minor_units: 2, symbol: None },
    Currency { code: "TJS", minor_units: 2, symbol: None },
    Currency { code: "TMT", minor_units: 2, symbol: None },
    Currency { code: "TOP", minor_units: 2, symbol: None },
    Currency { code: "TTD", minor_units: 2, symbol: None },
    Currency { code: "TZS", minor_units: 2, symbol: None },
    Currency { code: "UZS", minor_units: 2, symbol: None },
    Currency { code: "WST", minor_units: 2, symbol: None },
    Currency { code: "XCD", minor_units: 2, symbol: None },
    Currency { code: "YER", minor_units: 2, symbol: None },
    Currency { code: "ZMW", minor_units: 2, symbol: None },
    Currency { code: "ZWG", minor_units: 2, symbol: None },
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

    #[test]
    fn currency_table_has_no_duplicate_codes() {
        for (i, a) in CURRENCIES.iter().enumerate() {
            for b in &CURRENCIES[i + 1..] {
                assert_ne!(a.code, b.code, "duplicate currency code {}", a.code);
            }
        }
    }

    #[test]
    fn currency_table_has_no_duplicate_symbols() {
        let symbols: Vec<char> = CURRENCIES.iter().filter_map(|c| c.symbol).collect();
        for (i, a) in symbols.iter().enumerate() {
            assert!(
                !symbols[i + 1..].contains(a),
                "symbol '{}' is assigned to more than one currency",
                a
            );
        }
    }

    #[test]
    fn zero_decimal_currencies_are_correctly_classified() {
        for code in ["JPY", "KRW", "VND", "CLP", "ISK"] {
            assert_eq!(by_code(code).unwrap().minor_units, 0, "{} should have 0 minor units", code);
        }
    }

    #[test]
    fn three_decimal_currencies_are_correctly_classified() {
        for code in ["BHD", "KWD", "OMR", "JOD", "TND", "IQD", "LYD"] {
            assert_eq!(by_code(code).unwrap().minor_units, 3, "{} should have 3 minor units", code);
        }
    }

    #[test]
    fn recently_added_currencies_are_looked_up_case_insensitively() {
        for code in ["AFN", "XCD", "ZWG", "MRU", "STN", "SLE"] {
            assert_eq!(by_code(code).unwrap().code, code);
            assert_eq!(by_code(&code.to_lowercase()).unwrap().code, code);
        }
    }
}
