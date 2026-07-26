//! Domain modeling with currency-tagged positive money and sum types.
//!
//! `PositiveMoney` proves nonzero minor units and one explicit currency. It
//! does not encode tax, foreign exchange, allocation, or rounding policy.

use std::error::Error;
use std::fmt;
use std::num::NonZeroU64;

/// Currency attached to every money value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Currency {
    /// Indonesian rupiah.
    Idr,
    /// United States dollar.
    Usd,
}

/// Failure to construct a positive amount.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoneyError {
    /// Zero is outside the positive-money invariant.
    Zero,
}

impl fmt::Display for MoneyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("money amount must be nonzero"),
        }
    }
}

impl Error for MoneyError {}

/// Failure to add two positive money values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdditionError {
    /// Currency tags differ.
    CurrencyMismatch {
        /// Currency of the left operand.
        left: Currency,
        /// Currency of the right operand.
        right: Currency,
    },
    /// Minor-unit addition exceeded `u64`.
    Overflow,
}

impl fmt::Display for AdditionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrencyMismatch { left, right } => {
                write!(formatter, "currency mismatch: {left:?} versus {right:?}")
            }
            Self::Overflow => formatter.write_str("money addition overflowed"),
        }
    }
}

impl Error for AdditionError {}

/// Nonzero minor units in one currency.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PositiveMoney {
    minor_units: NonZeroU64,
    currency: Currency,
}

impl PositiveMoney {
    /// Constructs positive money.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::Zero`] when `minor_units` is zero.
    pub fn new(minor_units: u64, currency: Currency) -> Result<Self, MoneyError> {
        let minor_units = NonZeroU64::new(minor_units).ok_or(MoneyError::Zero)?;
        Ok(Self {
            minor_units,
            currency,
        })
    }

    /// Returns the nonzero amount in the currency's minor unit.
    pub const fn minor_units(self) -> u64 {
        self.minor_units.get()
    }

    /// Returns the amount's currency tag.
    pub const fn currency(self) -> Currency {
        self.currency
    }

    /// Adds values only when currencies agree and arithmetic does not overflow.
    ///
    /// # Errors
    ///
    /// Returns [`AdditionError::CurrencyMismatch`] for unlike currencies and
    /// [`AdditionError::Overflow`] when the sum exceeds `u64`.
    pub fn checked_add(self, other: Self) -> Result<Self, AdditionError> {
        if self.currency != other.currency {
            return Err(AdditionError::CurrencyMismatch {
                left: self.currency,
                right: other.currency,
            });
        }

        let minor_units = self
            .minor_units
            .checked_add(other.minor_units.get())
            .ok_or(AdditionError::Overflow)?;

        Ok(Self {
            minor_units,
            currency: self.currency,
        })
    }
}

/// Provider or ledger receipt attached only to a paid invoice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptId(String);

impl ReceiptId {
    /// Constructs a nonempty receipt identifier.
    ///
    /// # Errors
    ///
    /// Returns [`IdentifierError`] for empty or whitespace-only input.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        let value = value.trim();
        if value.is_empty() {
            return Err(IdentifierError);
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Failure to construct an example identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdentifierError;

impl fmt::Display for IdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("identifier must not be empty")
    }
}

impl Error for IdentifierError {}

/// Structured reason attached only to a failed invoice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvoiceFailure {
    /// Invoice policy rejected the request.
    PolicyRejected,
    /// A named persistence conflict prevented the transition.
    VersionConflict,
}

/// Mutually exclusive invoice lifecycle state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvoiceState {
    /// Invoice awaits a terminal local state.
    Pending,
    /// Invoice was paid with receipt evidence.
    Paid {
        /// Receipt associated with the paid state.
        receipt: ReceiptId,
    },
    /// Invoice entered a confirmed local failure state.
    Failed {
        /// Structured reason associated with failure.
        reason: InvoiceFailure,
    },
}

#[cfg(test)]
mod tests {
    use super::{AdditionError, Currency, InvoiceState, MoneyError, PositiveMoney, ReceiptId};

    #[test]
    fn positive_money_rejects_zero() {
        assert_eq!(PositiveMoney::new(0, Currency::Idr), Err(MoneyError::Zero));
    }

    #[test]
    fn positive_money_adds_same_currency() {
        let first = PositiveMoney::new(2_000, Currency::Idr).expect("positive amount");
        let second = PositiveMoney::new(3_000, Currency::Idr).expect("positive amount");
        let total = first.checked_add(second).expect("same currency fits");

        assert_eq!(total.minor_units(), 5_000);
        assert_eq!(total.currency(), Currency::Idr);
    }

    #[test]
    fn addition_rejects_currency_mismatch() {
        let idr = PositiveMoney::new(1, Currency::Idr).expect("positive amount");
        let usd = PositiveMoney::new(1, Currency::Usd).expect("positive amount");

        assert_eq!(
            idr.checked_add(usd),
            Err(AdditionError::CurrencyMismatch {
                left: Currency::Idr,
                right: Currency::Usd,
            })
        );
    }

    #[test]
    fn paid_state_requires_a_receipt() {
        let receipt = ReceiptId::new("  receipt-42  ").expect("nonempty identifier");
        assert_eq!(receipt.as_str(), "receipt-42");
        let state = InvoiceState::Paid {
            receipt: receipt.clone(),
        };

        assert_eq!(state, InvoiceState::Paid { receipt });
    }
}
