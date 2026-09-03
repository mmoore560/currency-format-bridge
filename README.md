# currency-format-bridge

Ledgers and accounting systems want amounts as exact integers in a
currency's minor unit (cents, pence, fils) because floating point cannot
represent money reliably and even fixed-decimal arithmetic gets fiddly once
you cross currencies with different numbers of decimal places (USD has 2,
JPY has 0, BHD has 3). People, on the other hand, want to type and read
`$1,234.56`, not `USD 123456`.

`cfbridge` converts between those two representations, one amount per line:

- **display format**: `$1,234.56`, `-¥500`, `1234.56 CHF`
- **ledger format**: `USD 123456`, `JPY -500`, `CHF 123456`

## Usage

```
cfbridge --to-ledger amounts.txt
cfbridge --to-display ledger.txt
cfbridge --to-ledger < amounts.txt
cfbridge --to-display --locale=eu ledger.txt
```

By default, display format uses `,` for thousands grouping and `.` for the
decimal point (`1,234.56`). Pass `--locale=eu` to read and write the format
common across continental Europe instead, where those two roles are swapped
(`1.234,56`). Ledger format is unaffected either way — it has no separators
to be ambiguous about.

With `amounts.txt` containing:

```
$1,234.56
-¥500
12.34 EUR
```

`cfbridge --to-ledger amounts.txt` prints:

```
USD 123456
JPY -500
EUR 1234
```

## Error messages

Every parse failure reports the exact line and column of the problem, with
the offending line printed and a caret under the character:

```
$ echo '$12.5' | cfbridge --to-ledger
error: USD amounts use 2 fractional digit(s), but this amount has 1
  |
1 | $12.5
  |     ^
  at line 1, column 5
```

This matters once you are converting a file with hundreds of lines pulled
from somewhere else (an export, a pasted email, a spreadsheet) — "line 214
is wrong" is not useful, "line 214, column 9, the fractional part has the
wrong number of digits for BHD" is.

## Format details

- Display format accepts an optional leading `-`, then either a currency
  symbol prefix (`$`, `€`, `£`, `¥`) or a trailing ISO 4217 code after the
  number (`12.34 EUR`), never both.
- Thousands separators (`,`) are checked for correct grouping: the first
  group may have 1–3 digits, every group after it must have exactly 3.
- The fractional part, when present, must have exactly as many digits as
  the currency's minor unit count. Omitting it is only allowed when that
  is equivalent to writing zeros (`$12` means `$12.00`).
- Ledger format is `<CODE> <integer>`, where the integer is the exact
  number of minor units and carries the sign.

Around sixty currencies are built in, covering all three fractional-digit
counts ISO 4217 uses (0, 2, and 3) — see `src/amount.rs` for the full list.
Only USD, EUR, GBP, and JPY get a symbol; everything else is written with
a trailing code (`12.34 CAD`).

## Status

First pass. Unit tests cover the parser's edge cases and error column
math (`cargo test`). The currency table is bigger than a starter set now
but still short of the full ISO 4217 list. `--locale` covers the two
common separator conventions; round-trip validation and a totals/summary
mode grouped by currency code are next.
