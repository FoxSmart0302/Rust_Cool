/// Constructs a custom Excel formatter for various currencies.
///
/// See https://support.microsoft.com/en-au/office/number-format-codes-5026bbd6-04bc-48cd-bf33-80f18b4eae68
#[derive(Default)]
pub struct MoneyBuilder<'a> {
    lhs: Option<&'a str>,
    rhs: Option<&'a str>,
    zero: Option<&'a str>,
    decimal_places: Option<usize>,
}

// The format strings look worse than they are.
//
// For example:
// _("€"* #,##0.00 "EGP"_);_("€"* (#,##0.00) "EGP"_);_("€"* #,##0.00 "EGP"_)
// is composed of three parts, separated by semicolons.
//
// The first part is displayed for positive numbers,
// the second part for negative numbers,
// and the third part for zero.
//
// So
//   - _("€"* #,##0.00 "EGP"_); is for positive numbers
//   - _("€"* (#,##0.00) "EGP"_); is for negative numbers
//   - _("€"* #,##0.00 "EGP"_) is for zero
//
// Breaking that down further:
//   - the leading and trailing "_(" and "_)" are used to add one "(" or ")" worth of
//     space to the start and end of the displayed value.
//   - the "€" is the left hand side currency symbol. Enclosing in double quotes allow
//     multiple characters in here (like for R$ or S$).
//   - the "* " says to repeat the next character (a space in this case) as many times
//     as needed to fill the cell.
//   - the "#,##0.00" is the number format, with the comma separating thousands, and
//     the dot separating the decimal places. The number of decimal places is fixed
//     at 2.
//   - the "EGP" is the right hand side currency symbol.

impl<'a> MoneyBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The right hand side currency symbol. Can be multiple characters (ie "R$" or "EGP")
    pub fn rhs(mut self, rhs: &'a str) -> Self {
        self.rhs = Some(rhs);
        self
    }

    /// The left hand side currency symbol. Can be multiple characters (ie "R$" or "EGP")
    pub fn lhs(mut self, lhs: &'a str) -> Self {
        self.lhs = Some(lhs);
        self
    }

    /// Sets the number of decimal places to display. Defaults to 2.
    pub fn decimal_places(mut self, decimal_places: usize) -> Self {
        self.decimal_places = Some(decimal_places);
        self
    }

    /// Override the zero format string. By default, the zero value
    /// will be displayed the same as the positive format.
    ///
    /// For example, if you want a zero value to show an empty cell,
    /// you can set this to an empty string ("").
    pub fn zero(mut self, zero: &'a str) -> Self {
        self.zero = Some(zero);
        self
    }

    /// Remove the zero format string, so that it falls back to the
    /// default, which is the same as the positive format.
    pub fn default_zero(mut self) -> Self {
        self.zero = None;
        self
    }

    /// Build the format string.
    pub fn build(self) -> String {
        let inner = self.inner();

        // Both have an extra leading space.
        // The positive number has an extra space after the digits themselves, but before the RHS currency symbol
        let pos = format!("_({}{}_){}", self.get_lhs(), inner, self.get_rhs());
        // The negative number has it's digits enclosed in parentheses.
        let neg = format!("_({}({}){}", self.get_lhs(), inner, self.get_rhs());

        let zero = match self.zero {
            Some(z) => z,
            None => &pos,
        };

        format!("{};{};{}", pos, neg, zero)
    }

    fn get_lhs(&self) -> String {
        match self.lhs {
            Some(lhs) => format!(r#""{}"* "#, lhs),
            None => String::new(),
        }
    }

    fn get_rhs(&self) -> String {
        match self.rhs {
            Some(rhs) => format!(r#" "{}""#, rhs),
            None => String::new(),
        }
    }

    /// The inner format string. This deals with the actual digits of the number.
    ///   #,## = show thousands separator
    ///   0.00 = show two decimal places
    ///      0 = show no decimal places
    fn inner(&self) -> String {
        match self.decimal_places {
            None => "#,##0.00".to_string(),
            Some(n) if n == 0 => "#,##0".to_string(),
            Some(n) => format!("#,##0.{}", "0".repeat(n)),
        }
    }
}

#[cfg(test)]
mod money_builder_tests {
    use crate::xlsx::format::money_builder::MoneyBuilder;

    #[test]
    fn it_works_with_both() {
        let b = MoneyBuilder::new().lhs(r#"€"#).rhs("EGP").build();

        assert_eq!(
            r#"_("€"* #,##0.00_) "EGP";_("€"* (#,##0.00) "EGP";_("€"* #,##0.00_) "EGP""#,
            b
        );
    }

    #[test]
    fn it_works_with_decimal_places() {
        let b = MoneyBuilder::new().lhs("¥").decimal_places(0).build();

        assert_eq!(r#"_("¥"* #,##0_);_("¥"* (#,##0);_("¥"* #,##0_)"#, b);
    }
}
