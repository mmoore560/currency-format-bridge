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
cfbridge --validate amounts.txt
cfbridge --totals amounts.txt
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

## Validation mode

`--validate` reads display-format lines and pushes each one through
`display -> ledger -> display`. Since the ledger format is an exact integer,
that round trip is lossless — so if the result doesn't match the original
line, the line was syntactically valid but not in canonical form (missing
thousands separators, an omitted fractional part, and the like):

```
$ printf '$1234.56\n$12\n$1,234.56\n' | cfbridge --validate
1: not canonical, canonical form is '$1,234.56'
2: not canonical, canonical form is '$12.00'
3: ok
```

Lines that fail to parse at all are reported the same way `--to-ledger`
does, with the line/column caret. `--validate` exits non-zero if any line
is a parse error or not already canonical.

## Totals mode

`--totals` reads display-format lines and prints one summed line per
currency code, sorted by code, formatted the same way `--to-display` would
format any other amount (so `--locale` applies here too):

```
$ printf '$10.00\n$5.50\n-¥100\n' | cfbridge --totals
-¥100
$15.50
```

Lines that fail to parse are reported with the usual line/column caret and
excluded from the totals; `--totals` exits non-zero if any line failed to
parse or a running total overflowed a 64-bit integer.

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
common separator conventions, `--validate` covers round-trip checking, and
`--totals` covers summing by currency code.
