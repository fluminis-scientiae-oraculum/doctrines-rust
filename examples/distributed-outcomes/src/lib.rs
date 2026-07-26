//! Explicit external outcomes and retry decisions.

use std::error::Error;
use std::fmt;

/// Failure to construct a stable operation identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdentityError;

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("identity must not be empty")
    }
}

impl Error for IdentityError {}

/// Stable identity for one logical operation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OperationId(String);

impl OperationId {
    /// Constructs a nonempty operation identity.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityError`] for empty or whitespace-only input.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        nonempty(value).map(Self)
    }

    /// Returns the identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Receiver-scoped idempotency identity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Constructs a nonempty idempotency key.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityError`] for empty or whitespace-only input.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        nonempty(value).map(Self)
    }

    /// Returns the key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Evidence required to observe an ambiguous external operation later.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationToken {
    operation_id: OperationId,
    provider_reference: String,
}

impl ReconciliationToken {
    /// Constructs reconciliation evidence.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityError`] when the provider reference is empty.
    pub fn new(
        operation_id: OperationId,
        provider_reference: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        Ok(Self {
            operation_id,
            provider_reference: nonempty(provider_reference)?,
        })
    }

    /// Returns the stable operation identity.
    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    /// Returns the provider lookup reference.
    pub fn provider_reference(&self) -> &str {
        &self.provider_reference
    }
}

/// Outcome of an external operation with possible execution ambiguity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationOutcome<T, E> {
    /// Authoritative evidence confirmed success.
    Confirmed(T),
    /// Authoritative evidence confirmed rejection.
    Rejected(E),
    /// Available evidence cannot establish success or rejection.
    Unknown {
        /// Evidence needed for later observation.
        reconciliation: ReconciliationToken,
    },
}

/// Observation at the failure point of an attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptObservation {
    /// Transport evidence proves no request crossed the dispatch boundary.
    NotDispatched,
    /// Remote source confirmed rejection.
    ConfirmedRejection,
    /// Request may have crossed the execution boundary.
    ExecutionAmbiguous,
}

/// Safe next action for one logical operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetryDecision {
    /// Reuse the same operation and idempotency identity within its budget.
    RetrySameIdentity,
    /// Observe authoritative external state before another attempt.
    ReconcileBeforeRetry,
    /// Do not repeat the operation automatically.
    DoNotRetry,
}

/// Chooses retry behavior from failure-point evidence and receiver semantics.
pub const fn decide_retry(
    observation: AttemptObservation,
    receiver_is_idempotent: bool,
) -> RetryDecision {
    match observation {
        AttemptObservation::NotDispatched => RetryDecision::RetrySameIdentity,
        AttemptObservation::ConfirmedRejection => RetryDecision::DoNotRetry,
        AttemptObservation::ExecutionAmbiguous if receiver_is_idempotent => {
            RetryDecision::RetrySameIdentity
        }
        AttemptObservation::ExecutionAmbiguous => RetryDecision::ReconcileBeforeRetry,
    }
}

fn nonempty(value: impl Into<String>) -> Result<String, IdentityError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(IdentityError);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        AttemptObservation, OperationId, OperationOutcome, ReconciliationToken, RetryDecision,
        decide_retry,
    };

    #[test]
    fn ambiguous_non_idempotent_attempt_requires_reconciliation() {
        assert_eq!(
            decide_retry(AttemptObservation::ExecutionAmbiguous, false),
            RetryDecision::ReconcileBeforeRetry
        );
    }

    #[test]
    fn pre_dispatch_failure_reuses_logical_identity() {
        assert_eq!(
            decide_retry(AttemptObservation::NotDispatched, false),
            RetryDecision::RetrySameIdentity
        );
    }

    #[test]
    fn unknown_outcome_does_not_become_rejection() {
        let operation = OperationId::new("capture-42").expect("nonempty operation");
        let reconciliation = ReconciliationToken::new(operation.clone(), "provider-capture-7")
            .expect("nonempty provider reference");
        let outcome: OperationOutcome<&str, &str> = OperationOutcome::Unknown {
            reconciliation: reconciliation.clone(),
        };

        assert_eq!(outcome, OperationOutcome::Unknown { reconciliation });
        assert_eq!(operation.as_str(), "capture-42");
    }
}
