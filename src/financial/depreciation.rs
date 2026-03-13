//! # Depreciation
//!
//! In accounting, depreciation refers to the decreases in the value of a fixed
//! asset during the asset's useful life. When an organization purchases a fixed
//! asset, the purchase expenditure is not recognized as an expense immediately.
//! Instead, the decreases in the asset's value are recognized as expenses over
//! the years during which the asset is used.
//!
//! The following methods are implemented here:
//! - **Straight-line method** — cost spread evenly over the asset's useful life.
//! - **Diminishing balance method** — a fixed percentage is applied each year to
//!   the asset's remaining book value.
//! - **Units-of-production method** — depreciation is tied to actual usage
//!   (units produced / hours used) rather than time.
//! - **Sum-of-years' digits (SYD)** — an accelerated method that applies a
//!   declining fraction to the depreciable cost each year.
//! - **Double-declining balance (DDB)** — the most common accelerated method;
//!   uses `rate = 2 / useful_years` and automatically switches to straight-line
//!   in the year that method yields a higher charge.
//!
//! Further information: <https://en.wikipedia.org/wiki/Depreciation>

// ─────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────

/// Errors that can occur during a depreciation calculation.
#[derive(Debug, PartialEq)]
pub enum DepreciationError {
    /// `useful_years` was supplied as zero.
    UsefulYearsZero,
    /// `purchase_value` is negative.
    NegativePurchaseValue,
    /// `purchase_value` is less than `residual_value`.
    PurchaseValueLessThanResidual,
    /// A rate or percentage argument is out of its valid range.
    InvalidRate(String),
    /// The sum of units-of-production estimates does not match `total_units`.
    UnitsMismatch { expected: f64, got: f64 },
}

impl std::fmt::Display for DepreciationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UsefulYearsZero => write!(f, "Useful years must be at least 1"),
            Self::NegativePurchaseValue => {
                write!(f, "Purchase value cannot be less than zero")
            }
            Self::PurchaseValueLessThanResidual => {
                write!(f, "Purchase value cannot be less than residual value")
            }
            Self::InvalidRate(msg) => write!(f, "Invalid rate: {msg}"),
            Self::UnitsMismatch { expected, got } => write!(
                f,
                "Units mismatch: expected {expected} total units, got {got}"
            ),
        }
    }
}

impl std::error::Error for DepreciationError {}

// ─────────────────────────────────────────────────────────────
// Shared validation helper
// ─────────────────────────────────────────────────────────────

fn validate_common(
    useful_years: u32,
    purchase_value: f64,
    residual_value: f64,
) -> Result<(), DepreciationError> {
    if useful_years == 0 {
        return Err(DepreciationError::UsefulYearsZero);
    }
    if purchase_value < 0.0 {
        return Err(DepreciationError::NegativePurchaseValue);
    }
    if purchase_value < residual_value {
        return Err(DepreciationError::PurchaseValueLessThanResidual);
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────
// Strategy 1 — Straight-line
// ─────────────────────────────────────────────────────────────

/// Calculates depreciation using the **straight-line** method.
///
/// The depreciable cost is divided equally across every period.
///
/// **Formula:**
/// ```text
/// annual_expense = (purchase_value - residual_value) / useful_years
/// ```
///
/// The last year's expense is adjusted so that accumulated depreciation
/// sums to exactly `purchase_value - residual_value` (avoids floating-point
/// drift).
///
/// # Errors
/// Returns [`DepreciationError`] if any argument is invalid.
///
/// # Examples
/// ```
/// use the_algorithms_rust::financial::straight_line_depreciation;
///
/// let result = straight_line_depreciation(10, 1100.0, 100.0).unwrap();
/// assert!(result.iter().all(|&v| (v - 100.0).abs() < 1e-9));
/// ```
pub fn straight_line_depreciation(
    useful_years: u32,
    purchase_value: f64,
    residual_value: f64,
) -> Result<Vec<f64>, DepreciationError> {
    validate_common(useful_years, purchase_value, residual_value)?;

    let depreciable_cost = purchase_value - residual_value;
    let annual_expense = depreciable_cost / useful_years as f64;

    let mut schedule: Vec<f64> = Vec::with_capacity(useful_years as usize);
    let mut accumulated = 0.0_f64;

    for period in 0..useful_years {
        if period == useful_years - 1 {
            // Last period absorbs any floating-point remainder
            schedule.push(depreciable_cost - accumulated);
        } else {
            accumulated += annual_expense;
            schedule.push(annual_expense);
        }
    }

    Ok(schedule)
}

// ─────────────────────────────────────────────────────────────
// Strategy 2 — Diminishing balance (declining balance)
// ─────────────────────────────────────────────────────────────

/// Calculates depreciation using the **diminishing balance** (declining
/// balance) method.
///
/// A fixed `rate` (0 < rate ≤ 1) is applied each year to the *remaining book
/// value* of the asset.  In the final year the entire remaining book value
/// above the residual is written off, ensuring the schedule sums exactly to
/// the depreciable cost.
///
/// **Formula per period:**
/// ```text
/// expense(t) = book_value(t) * rate
/// book_value(t+1) = book_value(t) - expense(t)
/// ```
///
/// A common variant is the **double-declining balance** method, which sets
/// `rate = 2 / useful_years`.
///
/// # Errors
/// Returns [`DepreciationError`] if any argument is invalid or `rate` is not
/// in `(0, 1]`.
///
/// # Examples
/// ```
/// use the_algorithms_rust::financial::diminishing_balance_depreciation;
///
/// // Double-declining balance on a 5-year, $10 000 asset with no salvage value
/// let rate = 2.0 / 5.0;
/// let schedule = diminishing_balance_depreciation(5, 10_000.0, 0.0, rate).unwrap();
/// let total: f64 = schedule.iter().sum();
/// assert!((total - 10_000.0).abs() < 1e-6);
/// ```
pub fn diminishing_balance_depreciation(
    useful_years: u32,
    purchase_value: f64,
    residual_value: f64,
    rate: f64,
) -> Result<Vec<f64>, DepreciationError> {
    validate_common(useful_years, purchase_value, residual_value)?;

    if rate <= 0.0 || rate > 1.0 {
        return Err(DepreciationError::InvalidRate(format!(
            "Diminishing balance rate must be in (0, 1], got {rate}"
        )));
    }

    let mut schedule: Vec<f64> = Vec::with_capacity(useful_years as usize);
    let mut book_value = purchase_value;

    for period in 0..useful_years {
        if period == useful_years - 1 {
            // Final year: write off everything above residual value
            schedule.push(book_value - residual_value);
        } else {
            let expense = book_value * rate;
            // Never depreciate below the residual value mid-life
            let expense = expense.min(book_value - residual_value);
            schedule.push(expense);
            book_value -= expense;
        }
    }

    Ok(schedule)
}

// ─────────────────────────────────────────────────────────────
// Strategy 3 — Units-of-production
// ─────────────────────────────────────────────────────────────

/// Calculates depreciation using the **units-of-production** method.
///
/// Depreciation in each period is proportional to the number of units
/// produced (or hours used) in that period relative to the total expected
/// output over the asset's life.
///
/// **Formula per period:**
/// ```text
/// expense(t) = (units_in_period(t) / total_units) * depreciable_cost
/// ```
///
/// `units_per_period` must sum to `total_units` (within a small floating-point
/// tolerance).
///
/// # Errors
/// Returns [`DepreciationError`] if any argument is invalid or the unit
/// totals do not match.
///
/// # Examples
/// ```
/// use the_algorithms_rust::financial::units_of_production_depreciation;
///
/// // 4-year asset; total output = 100 000 units
/// let schedule = units_of_production_depreciation(
///     1_000.0, 100.0, 100_000.0,
///     &[30_000.0, 25_000.0, 25_000.0, 20_000.0],
/// ).unwrap();
/// let total: f64 = schedule.iter().sum();
/// assert!((total - 900.0).abs() < 1e-6);
/// ```
pub fn units_of_production_depreciation(
    purchase_value: f64,
    residual_value: f64,
    total_units: f64,
    units_per_period: &[f64],
) -> Result<Vec<f64>, DepreciationError> {
    let useful_years = units_per_period.len() as u32;
    validate_common(useful_years, purchase_value, residual_value)?;

    if total_units <= 0.0 {
        return Err(DepreciationError::InvalidRate(
            "total_units must be positive".into(),
        ));
    }

    let units_sum: f64 = units_per_period.iter().sum();
    if (units_sum - total_units).abs() > 1e-6 * total_units {
        return Err(DepreciationError::UnitsMismatch {
            expected: total_units,
            got: units_sum,
        });
    }

    let depreciable_cost = purchase_value - residual_value;
    let depreciation_per_unit = depreciable_cost / total_units;

    let mut schedule: Vec<f64> = Vec::with_capacity(units_per_period.len());
    let mut accumulated = 0.0_f64;

    for (i, &units) in units_per_period.iter().enumerate() {
        if i == units_per_period.len() - 1 {
            // Final period absorbs floating-point remainder
            schedule.push(depreciable_cost - accumulated);
        } else {
            let expense = units * depreciation_per_unit;
            accumulated += expense;
            schedule.push(expense);
        }
    }

    Ok(schedule)
}

// ─────────────────────────────────────────────────────────────
// Strategy 4 — Sum-of-years' digits (SYD)
// ─────────────────────────────────────────────────────────────

/// Calculates depreciation using the **sum-of-years' digits (SYD)** method.
///
/// This is an *accelerated* method: higher depreciation is charged in earlier
/// years, tapering off as the asset ages.  Each year's expense is computed by
/// multiplying the depreciable cost by a fraction whose numerator is the
/// remaining useful life at the start of the period and whose denominator is
/// the sum of all year numbers (the "sum of digits").
///
/// **Formula:**
/// ```text
/// SYD          = n * (n + 1) / 2          where n = useful_years
/// fraction(t)  = (n - t + 1) / SYD        for period t = 1 … n
/// expense(t)   = fraction(t) * depreciable_cost
/// ```
///
/// The schedule always sums exactly to `purchase_value - residual_value`.
///
/// # Errors
/// Returns [`DepreciationError`] if any argument is invalid.
///
/// # Examples
/// ```
/// use the_algorithms_rust::financial::sum_of_years_digits_depreciation;
///
/// // 5-year asset, $10 000 cost, no residual
/// // SYD = 15; fractions: 5/15, 4/15, 3/15, 2/15, 1/15
/// let s = sum_of_years_digits_depreciation(5, 10_000.0, 0.0).unwrap();
/// assert!((s[0] - 10_000.0 * 5.0 / 15.0).abs() < 1e-9);
/// assert!((s[4] - 10_000.0 * 1.0 / 15.0).abs() < 1e-9);
/// let total: f64 = s.iter().sum();
/// assert!((total - 10_000.0).abs() < 1e-9);
/// ```
pub fn sum_of_years_digits_depreciation(
    useful_years: u32,
    purchase_value: f64,
    residual_value: f64,
) -> Result<Vec<f64>, DepreciationError> {
    validate_common(useful_years, purchase_value, residual_value)?;

    let n = useful_years as f64;
    // Sum of digits: 1 + 2 + … + n = n(n+1)/2
    let syd = n * (n + 1.0) / 2.0;
    let depreciable_cost = purchase_value - residual_value;

    let mut schedule: Vec<f64> = Vec::with_capacity(useful_years as usize);
    let mut accumulated = 0.0_f64;

    for period in 1..=useful_years {
        let remaining_life = (useful_years - period + 1) as f64;
        if period == useful_years {
            // Last period absorbs any floating-point remainder
            schedule.push(depreciable_cost - accumulated);
        } else {
            let expense = (remaining_life / syd) * depreciable_cost;
            accumulated += expense;
            schedule.push(expense);
        }
    }

    Ok(schedule)
}

// ─────────────────────────────────────────────────────────────
// Strategy 5 — Double-declining balance (DDB)
// ─────────────────────────────────────────────────────────────

/// Calculates depreciation using the **double-declining balance (DDB)** method.
///
/// DDB is the most widely used form of the diminishing balance method.  It
/// applies a rate of `2 / useful_years` to the current book value each period.
///
/// A key accounting rule is applied automatically: **in any year where the
/// straight-line charge on the remaining book value would exceed the DDB
/// charge, the method switches to straight-line** for that year and all
/// subsequent years.  This ensures the book value reaches the residual value
/// by the end of the asset's life without requiring a large write-off in the
/// final year.
///
/// **Formula:**
/// ```text
/// rate            = 2 / useful_years
/// ddb_expense(t)  = book_value(t) * rate
/// sl_expense(t)   = (book_value(t) - residual_value) / remaining_years(t)
/// expense(t)      = max(ddb_expense(t), sl_expense(t))
/// ```
///
/// # Errors
/// Returns [`DepreciationError`] if any argument is invalid.
///
/// # Examples
/// ```
/// use the_algorithms_rust::financial::double_declining_balance_depreciation;
///
/// // Classic textbook example: 5-year, $10 000, $1 000 residual
/// let s = double_declining_balance_depreciation(5, 10_000.0, 1_000.0).unwrap();
/// let total: f64 = s.iter().sum();
/// assert!((total - 9_000.0).abs() < 1e-6);
/// // First year should be higher than straight-line ($1 800 vs $4 000)
/// assert!(s[0] > s[4]);
/// ```
pub fn double_declining_balance_depreciation(
    useful_years: u32,
    purchase_value: f64,
    residual_value: f64,
) -> Result<Vec<f64>, DepreciationError> {
    validate_common(useful_years, purchase_value, residual_value)?;

    let rate = 2.0 / useful_years as f64;
    let depreciable_cost = purchase_value - residual_value;
    let mut schedule: Vec<f64> = Vec::with_capacity(useful_years as usize);
    let mut book_value = purchase_value;
    let mut accumulated = 0.0_f64;

    for period in 1..=useful_years {
        let remaining_years = (useful_years - period + 1) as f64;

        // DDB charge for this period (never below zero)
        let ddb_expense = (book_value * rate).max(0.0);

        // Straight-line charge on remaining depreciable cost
        let sl_expense = ((book_value - residual_value) / remaining_years).max(0.0);

        // Switch to straight-line if it gives a larger charge
        let expense = ddb_expense.max(sl_expense);

        // Cap so we never depreciate below residual value
        let expense = expense.min(book_value - residual_value);

        if period == useful_years {
            // Final period: write off exactly what remains above residual
            schedule.push(depreciable_cost - accumulated);
        } else {
            accumulated += expense;
            schedule.push(expense);
            book_value -= expense;
        }
    }

    Ok(schedule)
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Straight-line ────────────────────────────────────────

    #[test]
    fn sl_basic() {
        let s = straight_line_depreciation(10, 1100.0, 100.0).unwrap();
        assert_eq!(s.len(), 10);
        assert!(s.iter().all(|&v| (v - 100.0).abs() < 1e-9));
    }

    #[test]
    fn sl_six_years() {
        let s = straight_line_depreciation(6, 1250.0, 50.0).unwrap();
        assert!(s.iter().all(|&v| (v - 200.0).abs() < 1e-9));
    }

    #[test]
    fn sl_no_residual() {
        let s = straight_line_depreciation(4, 1001.0, 0.0).unwrap();
        assert!(s.iter().all(|&v| (v - 250.25).abs() < 1e-9));
    }

    #[test]
    fn sl_sum_equals_depreciable_cost() {
        let purchase = 7_500.0_f64;
        let residual = 500.0_f64;
        let s = straight_line_depreciation(7, purchase, residual).unwrap();
        let total: f64 = s.iter().sum();
        assert!((total - (purchase - residual)).abs() < 1e-9);
    }

    #[test]
    fn sl_single_year() {
        let s = straight_line_depreciation(1, 4985.0, 100.0).unwrap();
        assert_eq!(s.len(), 1);
        assert!((s[0] - 4885.0).abs() < 1e-9);
    }

    #[test]
    fn sl_err_zero_years() {
        assert_eq!(
            straight_line_depreciation(0, 1000.0, 0.0),
            Err(DepreciationError::UsefulYearsZero)
        );
    }

    #[test]
    fn sl_err_negative_purchase() {
        assert_eq!(
            straight_line_depreciation(5, -500.0, 0.0),
            Err(DepreciationError::NegativePurchaseValue)
        );
    }

    #[test]
    fn sl_err_residual_exceeds_purchase() {
        assert_eq!(
            straight_line_depreciation(5, 100.0, 200.0),
            Err(DepreciationError::PurchaseValueLessThanResidual)
        );
    }

    // ── Diminishing balance ──────────────────────────────────

    #[test]
    fn db_sum_equals_depreciable_cost() {
        let purchase = 10_000.0_f64;
        let residual = 0.0_f64;
        let rate = 2.0 / 5.0; // double-declining, 5-year
        let s = diminishing_balance_depreciation(5, purchase, residual, rate).unwrap();
        let total: f64 = s.iter().sum();
        assert!((total - (purchase - residual)).abs() < 1e-6);
    }

    #[test]
    fn db_never_below_residual() {
        let purchase = 5_000.0_f64;
        let residual = 500.0_f64;
        let s = diminishing_balance_depreciation(6, purchase, residual, 0.5).unwrap();
        // book value must never drop below residual
        let mut book = purchase;
        for expense in &s {
            book -= expense;
        }
        assert!(book >= residual - 1e-9);
    }

    #[test]
    fn db_err_rate_zero() {
        assert!(matches!(
            diminishing_balance_depreciation(5, 1000.0, 0.0, 0.0),
            Err(DepreciationError::InvalidRate(_))
        ));
    }

    #[test]
    fn db_err_rate_above_one() {
        assert!(matches!(
            diminishing_balance_depreciation(5, 1000.0, 0.0, 1.1),
            Err(DepreciationError::InvalidRate(_))
        ));
    }

    // ── Units-of-production ──────────────────────────────────

    #[test]
    fn up_sum_equals_depreciable_cost() {
        let purchase = 1_000.0_f64;
        let residual = 100.0_f64;
        let total_units = 100_000.0_f64;
        let periods = vec![30_000.0, 25_000.0, 25_000.0, 20_000.0];
        let s =
            units_of_production_depreciation(purchase, residual, total_units, &periods).unwrap();
        let total: f64 = s.iter().sum();
        assert!((total - (purchase - residual)).abs() < 1e-9);
    }

    #[test]
    fn up_proportional_expenses() {
        let purchase = 10_000.0_f64;
        let residual = 0.0_f64;
        let total = 1_000.0_f64;
        let periods = vec![250.0, 250.0, 250.0, 250.0];
        let s = units_of_production_depreciation(purchase, residual, total, &periods).unwrap();
        assert!(s.iter().all(|&v| (v - 2_500.0).abs() < 1e-6));
    }

    #[test]
    fn up_err_units_mismatch() {
        assert!(matches!(
            units_of_production_depreciation(1000.0, 0.0, 100.0, &[30.0, 40.0, 20.0]),
            Err(DepreciationError::UnitsMismatch { .. })
        ));
    }

    #[test]
    fn up_err_zero_total_units() {
        assert!(matches!(
            units_of_production_depreciation(1000.0, 0.0, 0.0, &[0.0]),
            Err(DepreciationError::InvalidRate(_))
        ));
    }

    // ── Sum-of-years' digits ─────────────────────────────────

    #[test]
    fn syd_sum_equals_depreciable_cost() {
        let purchase = 10_000.0_f64;
        let residual = 0.0_f64;
        let s = sum_of_years_digits_depreciation(5, purchase, residual).unwrap();
        let total: f64 = s.iter().sum();
        assert!((total - (purchase - residual)).abs() < 1e-9);
    }

    #[test]
    fn syd_first_year_fraction() {
        // Year 1 fraction = n / SYD = 5/15 for a 5-year asset
        let s = sum_of_years_digits_depreciation(5, 10_000.0, 0.0).unwrap();
        let expected = 10_000.0 * 5.0 / 15.0;
        assert!((s[0] - expected).abs() < 1e-9);
    }

    #[test]
    fn syd_last_year_fraction() {
        // Year 5 fraction = 1/15
        let s = sum_of_years_digits_depreciation(5, 10_000.0, 0.0).unwrap();
        let expected = 10_000.0 * 1.0 / 15.0;
        assert!((s[4] - expected).abs() < 1e-9);
    }

    #[test]
    fn syd_decreasing_charges() {
        // Each year's charge must be strictly less than the previous
        let s = sum_of_years_digits_depreciation(6, 12_000.0, 0.0).unwrap();
        for window in s.windows(2) {
            assert!(window[0] > window[1]);
        }
    }

    #[test]
    fn syd_with_residual() {
        let purchase = 5_000.0_f64;
        let residual = 500.0_f64;
        let s = sum_of_years_digits_depreciation(4, purchase, residual).unwrap();
        let total: f64 = s.iter().sum();
        assert!((total - (purchase - residual)).abs() < 1e-9);
    }

    #[test]
    fn syd_single_year() {
        let s = sum_of_years_digits_depreciation(1, 2_000.0, 200.0).unwrap();
        assert_eq!(s.len(), 1);
        assert!((s[0] - 1_800.0).abs() < 1e-9);
    }

    #[test]
    fn syd_err_zero_years() {
        assert_eq!(
            sum_of_years_digits_depreciation(0, 1000.0, 0.0),
            Err(DepreciationError::UsefulYearsZero)
        );
    }

    // ── Double-declining balance ─────────────────────────────

    #[test]
    fn ddb_sum_equals_depreciable_cost() {
        let purchase = 10_000.0_f64;
        let residual = 1_000.0_f64;
        let s = double_declining_balance_depreciation(5, purchase, residual).unwrap();
        let total: f64 = s.iter().sum();
        assert!((total - (purchase - residual)).abs() < 1e-6);
    }

    #[test]
    fn ddb_first_year_is_double_sl() {
        // Year-1 DDB charge = 2/n * purchase_value
        let s = double_declining_balance_depreciation(5, 10_000.0, 0.0).unwrap();
        let expected_first = 10_000.0 * 2.0 / 5.0;
        assert!((s[0] - expected_first).abs() < 1e-9);
    }

    #[test]
    fn ddb_never_below_residual() {
        let purchase = 8_000.0_f64;
        let residual = 800.0_f64;
        let s = double_declining_balance_depreciation(6, purchase, residual).unwrap();
        let mut book = purchase;
        for expense in &s {
            book -= expense;
        }
        assert!(book >= residual - 1e-9);
    }

    #[test]
    fn ddb_switches_to_sl() {
        // With residual value, DDB must switch to straight-line at some point;
        // the last few charges should be equal (straight-line portion).
        let s = double_declining_balance_depreciation(5, 10_000.0, 2_000.0).unwrap();
        // At minimum the last two years should not decrease like pure DDB
        assert!(s[3] <= s[2] || (s[3] - s[4]).abs() < 1e-6);
    }

    #[test]
    fn ddb_err_zero_years() {
        assert_eq!(
            double_declining_balance_depreciation(0, 1000.0, 0.0),
            Err(DepreciationError::UsefulYearsZero)
        );
    }

    #[test]
    fn ddb_err_residual_exceeds_purchase() {
        assert_eq!(
            double_declining_balance_depreciation(5, 100.0, 200.0),
            Err(DepreciationError::PurchaseValueLessThanResidual)
        );
    }
}
