use std::collections::BTreeMap;

use crate::amount::Amount;

/// Running per-currency sums, keyed by ISO code so iteration order is
/// alphabetical without a separate sort step.
#[derive(Debug, Default)]
pub struct Totals {
    sums: BTreeMap<&'static str, i64>,
}

impl Totals {
    pub fn new() -> Self {
        Totals::default()
    }

    /// Add one amount's minor units to its currency's running total.
    /// Returns `Err(())` on i64 overflow so the caller can report it against
    /// the line that pushed the total over the edge.
    pub fn add(&mut self, amount: &Amount) -> Result<(), ()> {
        let entry = self.sums.entry(amount.currency.code).or_insert(0);
        *entry = entry.checked_add(amount.minor_units).ok_or(())?;
        Ok(())
    }

    /// (currency code, total minor units), ascending by code.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, i64)> + '_ {
        self.sums.iter().map(|(&code, &sum)| (code, sum))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amount::by_code;

    fn amount(code: &str, minor_units: i64) -> Amount {
        Amount { currency: by_code(code).unwrap(), minor_units }
    }

    #[test]
    fn sums_repeated_currency() {
        let mut totals = Totals::new();
        totals.add(&amount("USD", 100)).unwrap();
        totals.add(&amount("USD", 250)).unwrap();
        assert_eq!(totals.iter().collect::<Vec<_>>(), vec![("USD", 350)]);
    }

    #[test]
    fn keeps_currencies_separate_and_sorted_by_code() {
        let mut totals = Totals::new();
        totals.add(&amount("USD", 100)).unwrap();
        totals.add(&amount("JPY", 500)).unwrap();
        totals.add(&amount("EUR", 200)).unwrap();
        assert_eq!(
            totals.iter().collect::<Vec<_>>(),
            vec![("EUR", 200), ("JPY", 500), ("USD", 100)]
        );
    }

    #[test]
    fn negative_amounts_offset_the_total() {
        let mut totals = Totals::new();
        totals.add(&amount("USD", 500)).unwrap();
        totals.add(&amount("USD", -200)).unwrap();
        assert_eq!(totals.iter().collect::<Vec<_>>(), vec![("USD", 300)]);
    }

    #[test]
    fn empty_totals_has_no_entries() {
        let totals = Totals::new();
        assert_eq!(totals.iter().count(), 0);
    }

    #[test]
    fn overflow_is_reported_instead_of_wrapping() {
        let mut totals = Totals::new();
        totals.add(&amount("USD", i64::MAX)).unwrap();
        assert_eq!(totals.add(&amount("USD", 1)), Err(()));
    }
}
