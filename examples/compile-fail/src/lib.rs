//! Public API used by compile-fail evidence.

use std::marker::PhantomData;
use std::num::NonZeroU64;

/// Email address whose ownership proof was accepted by this example verifier.
#[derive(Debug, Eq, PartialEq)]
pub struct VerifiedEmailAddress {
    evidence: (String, String),
}

impl VerifiedEmailAddress {
    const fn from_accepted_evidence(address: String, verification_id: String) -> Self {
        Self {
            evidence: (address, verification_id),
        }
    }

    /// Returns the address associated with provider evidence.
    pub fn address(&self) -> &str {
        &self.evidence.0
    }

    /// Returns the example verification event identity.
    pub fn verification_id(&self) -> &str {
        &self.evidence.1
    }
}

/// Evidence created only by this crate's trusted provider adapter.
#[derive(Debug)]
pub struct ProviderOwnershipEvidence {
    address: String,
    verification_id: String,
}

impl ProviderOwnershipEvidence {
    #[cfg(test)]
    const fn recorded_by_provider(address: String, verification_id: String) -> Self {
        Self {
            address,
            verification_id,
        }
    }
}

/// Owner of the only public verified-email conversion path.
#[derive(Debug)]
pub struct EmailVerifier;

impl EmailVerifier {
    /// Converts provider-owned evidence into a verified address.
    ///
    /// This method demonstrates construction topology. A production adapter
    /// must authenticate the external verification evidence.
    pub fn accept_provider_evidence(evidence: ProviderOwnershipEvidence) -> VerifiedEmailAddress {
        VerifiedEmailAddress::from_accepted_evidence(evidence.address, evidence.verification_id)
    }
}

/// Closed connection marker.
#[derive(Debug)]
pub struct Closed;

/// Marker for successful local connection transition.
#[derive(Debug)]
pub struct Open;

/// State-specific local connection.
#[derive(Debug)]
pub struct Connection<State> {
    sequence: u64,
    state: PhantomData<State>,
}

impl Connection<Closed> {
    /// Creates a closed connection.
    pub const fn new() -> Self {
        Self {
            sequence: 0,
            state: PhantomData,
        }
    }

    /// Completes the example local connect transition.
    pub const fn connect(self) -> Connection<Open> {
        Connection {
            sequence: self.sequence,
            state: PhantomData,
        }
    }
}

impl Default for Connection<Closed> {
    fn default() -> Self {
        Self::new()
    }
}

impl Connection<Open> {
    /// Sends only through a locally open handle.
    ///
    /// # Errors
    ///
    /// Returns [`SendError`] when the receipt sequence is exhausted.
    pub fn send(&mut self, _payload: &[u8]) -> Result<SendReceipt, SendError> {
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or(SendError::SequenceExhausted)?;
        Ok(SendReceipt(self.sequence))
    }
}

/// Failure to issue a local receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendError {
    /// No later receipt sequence can be represented.
    SequenceExhausted,
}

/// Local receipt for compile-fail API evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SendReceipt(u64);

impl SendReceipt {
    /// Returns the receipt sequence.
    pub const fn sequence(self) -> u64 {
        self.0
    }
}

/// Draft payment marker.
#[derive(Debug)]
pub struct Draft;

/// Authorized local payment marker.
#[derive(Debug)]
pub struct Authorized;

/// Payment with state-specific operations.
#[derive(Debug)]
pub struct Payment<State> {
    amount: NonZeroU64,
    state: PhantomData<State>,
}

impl Payment<Draft> {
    /// Creates a positive draft payment.
    pub const fn new(amount: NonZeroU64) -> Self {
        Self {
            amount,
            state: PhantomData,
        }
    }

    /// Completes the example local authorization transition.
    pub const fn authorize(self) -> Payment<Authorized> {
        Payment {
            amount: self.amount,
            state: PhantomData,
        }
    }
}

impl Payment<Authorized> {
    /// Captures only an authorized local payment.
    pub const fn capture(self) -> CaptureReceipt {
        CaptureReceipt(self.amount)
    }
}

/// Local capture receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaptureReceipt(NonZeroU64);

impl CaptureReceipt {
    /// Returns the captured minor units.
    pub const fn amount(self) -> u64 {
        self.0.get()
    }
}

/// Active transaction whose commit consumes the handle.
#[derive(Debug, Default)]
pub struct Transaction {
    staged: Vec<String>,
}

impl Transaction {
    /// Begins an active transaction.
    pub const fn begin() -> Self {
        Self { staged: Vec::new() }
    }

    /// Stages a mutation while the handle is active.
    pub fn stage(&mut self, mutation: impl Into<String>) {
        self.staged.push(mutation.into());
    }

    /// Consumes the active handle.
    pub fn commit(self) -> TransactionReceipt {
        TransactionReceipt(self.staged.len())
    }
}

/// Local transaction receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransactionReceipt(usize);

impl TransactionReceipt {
    /// Returns the staged mutation count.
    pub const fn mutation_count(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Connection, EmailVerifier, Payment, ProviderOwnershipEvidence, Transaction};
    use std::num::NonZeroU64;

    #[test]
    fn legal_programs_compile_and_run() {
        let evidence = ProviderOwnershipEvidence::recorded_by_provider(
            "owner@example.com".to_owned(),
            "verification-1".to_owned(),
        );
        let verified = EmailVerifier::accept_provider_evidence(evidence);
        assert_eq!(verified.address(), "owner@example.com");

        let mut open = Connection::new().connect();
        assert_eq!(
            open.send(b"data").expect("first receipt fits").sequence(),
            1
        );

        let amount = NonZeroU64::new(1).expect("one is nonzero");
        assert_eq!(Payment::new(amount).authorize().capture().amount(), 1);

        let mut transaction = Transaction::begin();
        transaction.stage("mutation");
        assert_eq!(transaction.commit().mutation_count(), 1);
    }
}
