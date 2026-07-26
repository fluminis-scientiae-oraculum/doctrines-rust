//! Typestate for a small locally controlled protocol.
//!
//! `Connection<Open>` proves only that this value came from a successful local
//! `connect` transition. The peer can fail before the next `send`.

use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

/// Marker for a locally closed connection.
#[derive(Debug)]
pub struct Closed;

/// Marker for a connection whose local connect transition succeeded.
#[derive(Debug)]
pub struct Open;

/// Connection handle with state-specific methods.
#[derive(Debug)]
pub struct Connection<State> {
    endpoint: String,
    next_sequence: u64,
    state: PhantomData<State>,
}

/// Local connection transition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectError {
    /// Endpoint was empty.
    EmptyEndpoint,
    /// The example transport rejected the endpoint.
    Unreachable,
}

impl fmt::Display for ConnectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEndpoint => formatter.write_str("endpoint is empty"),
            Self::Unreachable => formatter.write_str("endpoint is unreachable"),
        }
    }
}

impl Error for ConnectError {}

/// Runtime send failure after a local open transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendError {
    /// Payload was empty.
    EmptyPayload,
    /// Peer became unavailable.
    RemoteUnavailable,
    /// Receipt sequence overflowed.
    SequenceExhausted,
}

impl fmt::Display for SendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPayload => formatter.write_str("payload is empty"),
            Self::RemoteUnavailable => formatter.write_str("remote peer is unavailable"),
            Self::SequenceExhausted => formatter.write_str("receipt sequence is exhausted"),
        }
    }
}

impl Error for SendError {}

/// Local close failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloseError {
    /// The example transport could not confirm local cleanup.
    LocalCleanupFailed,
}

impl fmt::Display for CloseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("local connection cleanup failed")
    }
}

impl Error for CloseError {}

/// Receipt for one example send attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SendReceipt {
    sequence: u64,
}

impl SendReceipt {
    /// Returns the local receipt sequence.
    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}

impl Connection<Closed> {
    /// Creates a locally closed connection handle.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            next_sequence: 1,
            state: PhantomData,
        }
    }

    /// Attempts the local connect transition.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectError`] when the endpoint is empty or configured as
    /// unreachable by this deterministic example.
    pub fn connect(self) -> Result<Connection<Open>, ConnectError> {
        if self.endpoint.trim().is_empty() {
            return Err(ConnectError::EmptyEndpoint);
        }
        if self.endpoint.starts_with("unreachable:") {
            return Err(ConnectError::Unreachable);
        }

        Ok(Connection {
            endpoint: self.endpoint,
            next_sequence: self.next_sequence,
            state: PhantomData,
        })
    }
}

impl Connection<Open> {
    /// Sends bytes through the currently owned transport.
    ///
    /// # Errors
    ///
    /// Returns [`SendError`] for an empty payload, a peer failure configured by
    /// the example endpoint, or receipt-sequence exhaustion.
    pub fn send(&mut self, payload: &[u8]) -> Result<SendReceipt, SendError> {
        if payload.is_empty() {
            return Err(SendError::EmptyPayload);
        }
        if self.endpoint.starts_with("drop-after-open:") {
            return Err(SendError::RemoteUnavailable);
        }

        let receipt = SendReceipt {
            sequence: self.next_sequence,
        };
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(SendError::SequenceExhausted)?;
        Ok(receipt)
    }

    /// Consumes the open handle and attempts local cleanup.
    ///
    /// # Errors
    ///
    /// Returns [`CloseError`] when the example endpoint models a cleanup
    /// failure. The prior open handle remains consumed.
    pub fn close(self) -> Result<Connection<Closed>, CloseError> {
        if self.endpoint.starts_with("close-fails:") {
            return Err(CloseError::LocalCleanupFailed);
        }

        Ok(Connection {
            endpoint: self.endpoint,
            next_sequence: self.next_sequence,
            state: PhantomData,
        })
    }
}

/// Locally active transaction workflow.
#[derive(Debug)]
pub struct ActiveTransaction {
    staged: Vec<String>,
}

/// Failure to stage a transaction mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageError {
    /// Empty statements are outside the example protocol.
    EmptyMutation,
}

impl fmt::Display for StageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("transaction mutation is empty")
    }
}

impl Error for StageError {}

/// Failure to commit the example transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommitError {
    /// No mutation was staged.
    NothingStaged,
}

impl fmt::Display for CommitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("transaction contains no staged mutation")
    }
}

impl Error for CommitError {}

/// Evidence that the local example commit transition completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommitReceipt {
    mutation_count: usize,
}

impl CommitReceipt {
    /// Returns the number of staged mutations in this local receipt.
    pub const fn mutation_count(self) -> usize {
        self.mutation_count
    }
}

/// Evidence that the local rollback transition completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RolledBack;

impl ActiveTransaction {
    /// Begins an empty local transaction.
    pub const fn begin() -> Self {
        Self { staged: Vec::new() }
    }

    /// Adds a mutation while the transaction remains active.
    ///
    /// # Errors
    ///
    /// Returns [`StageError::EmptyMutation`] for empty input.
    pub fn stage(&mut self, mutation: impl Into<String>) -> Result<(), StageError> {
        let mutation = mutation.into();
        if mutation.trim().is_empty() {
            return Err(StageError::EmptyMutation);
        }
        self.staged.push(mutation);
        Ok(())
    }

    /// Consumes the active handle and completes the local commit transition.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::NothingStaged`] when there is no mutation.
    pub fn commit(self) -> Result<CommitReceipt, CommitError> {
        if self.staged.is_empty() {
            return Err(CommitError::NothingStaged);
        }
        Ok(CommitReceipt {
            mutation_count: self.staged.len(),
        })
    }

    /// Consumes the active handle and completes local rollback.
    pub fn rollback(self) -> RolledBack {
        drop(self);
        RolledBack
    }
}

#[cfg(test)]
mod tests {
    use super::{ActiveTransaction, Closed, ConnectError, Connection, SendError, StageError};

    #[test]
    fn open_only_follows_successful_connect() {
        let connection = Connection::<Closed>::new("service:443")
            .connect()
            .expect("example endpoint connects");
        let closed = connection.close().expect("example endpoint closes");
        assert_eq!(closed.endpoint, "service:443");
    }

    #[test]
    fn connect_and_send_remain_fallible() {
        assert!(matches!(
            Connection::<Closed>::new("unreachable:service").connect(),
            Err(ConnectError::Unreachable)
        ));

        let mut connection = Connection::<Closed>::new("drop-after-open:service")
            .connect()
            .expect("local transition succeeds");
        assert_eq!(
            connection.send(b"invoice"),
            Err(SendError::RemoteUnavailable)
        );
    }

    #[test]
    fn transaction_consumes_terminal_state() {
        let mut transaction = ActiveTransaction::begin();
        transaction
            .stage("update account version")
            .expect("nonempty mutation");
        let receipt = transaction.commit().expect("mutation can commit");

        assert_eq!(receipt.mutation_count(), 1);
    }

    #[test]
    fn transaction_rejects_empty_mutation() {
        let mut transaction = ActiveTransaction::begin();
        assert_eq!(transaction.stage(" "), Err(StageError::EmptyMutation));
    }
}
