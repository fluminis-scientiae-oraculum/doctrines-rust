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

/// Stable identities carried by every attempt of one logical operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttemptIdentity {
    operation_id: OperationId,
    idempotency_key: IdempotencyKey,
}

impl AttemptIdentity {
    /// Associates one logical operation with its receiver-scoped idempotency key.
    pub const fn new(operation_id: OperationId, idempotency_key: IdempotencyKey) -> Self {
        Self {
            operation_id,
            idempotency_key,
        }
    }

    /// Returns the logical operation identity.
    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    /// Returns the key that every retry of this logical operation must reuse.
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetryDecision {
    /// Retry while preserving both identities from the prior attempt.
    RetrySameIdentity {
        /// Identity that the next attempt must carry unchanged.
        attempt: AttemptIdentity,
    },
    /// Observe authoritative external state before another attempt.
    ReconcileBeforeRetry {
        /// Logical operation whose external state must be reconciled.
        operation_id: OperationId,
    },
    /// Do not repeat the operation automatically.
    DoNotRetry,
}

/// Chooses retry behavior from failure-point evidence and receiver semantics.
pub fn decide_retry(
    attempt: &AttemptIdentity,
    observation: AttemptObservation,
    receiver_is_idempotent: bool,
) -> RetryDecision {
    match observation {
        AttemptObservation::NotDispatched => RetryDecision::RetrySameIdentity {
            attempt: attempt.clone(),
        },
        AttemptObservation::ConfirmedRejection => RetryDecision::DoNotRetry,
        AttemptObservation::ExecutionAmbiguous if receiver_is_idempotent => {
            RetryDecision::RetrySameIdentity {
                attempt: attempt.clone(),
            }
        }
        AttemptObservation::ExecutionAmbiguous => RetryDecision::ReconcileBeforeRetry {
            operation_id: attempt.operation_id.clone(),
        },
    }
}

fn nonempty(value: impl Into<String>) -> Result<String, IdentityError> {
    let value = value.into();
    let value = value.trim();
    if value.is_empty() {
        return Err(IdentityError);
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        AttemptIdentity, AttemptObservation, IdempotencyKey, OperationId, OperationOutcome,
        ReconciliationToken, RetryDecision, decide_retry,
    };

    fn attempt(operation: &str, key: &str) -> AttemptIdentity {
        AttemptIdentity::new(
            OperationId::new(operation).expect("nonempty operation"),
            IdempotencyKey::new(key).expect("nonempty idempotency key"),
        )
    }

    #[test]
    fn ambiguous_non_idempotent_attempt_requires_reconciliation() {
        let attempt = attempt("capture-42", "capture-key-42");
        assert_eq!(
            decide_retry(&attempt, AttemptObservation::ExecutionAmbiguous, false),
            RetryDecision::ReconcileBeforeRetry {
                operation_id: attempt.operation_id().clone(),
            }
        );
    }

    #[test]
    fn retries_reuse_the_same_operation_and_idempotency_key() {
        let first_attempt = attempt("capture-42", "capture-key-42");
        let decision = decide_retry(&first_attempt, AttemptObservation::NotDispatched, false);
        let RetryDecision::RetrySameIdentity { attempt: retry } = decision else {
            panic!("pre-dispatch failure should permit retry");
        };

        assert_eq!(retry, first_attempt);
        assert_eq!(retry.idempotency_key().as_str(), "capture-key-42");

        let different_operation = attempt("capture-43", "capture-key-43");
        assert_ne!(retry, different_operation);
        assert_ne!(
            retry.idempotency_key(),
            different_operation.idempotency_key()
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

    #[test]
    fn identities_normalize_surrounding_whitespace() {
        let operation = OperationId::new("  capture-42  ").expect("nonempty operation");
        let key = IdempotencyKey::new("  capture-key-42  ").expect("nonempty key");
        let reconciliation = ReconciliationToken::new(operation.clone(), "  provider-capture-7  ")
            .expect("nonempty provider reference");

        assert_eq!(operation.as_str(), "capture-42");
        assert_eq!(key.as_str(), "capture-key-42");
        assert_eq!(reconciliation.provider_reference(), "provider-capture-7");
    }
}
