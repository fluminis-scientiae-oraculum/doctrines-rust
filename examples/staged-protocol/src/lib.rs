//! Staged protocol whose successor capability is part of each stage contract.
//!
//! Every stage trait names its legal successor as an associated type bounded by
//! the capability that successor must satisfy. The bound is the protocol edge:
//! removing or redirecting it changes what the compiler accepts, so the stage
//! graph cannot drift away from its documentation without a compiler error.
//!
//! Two entry stages implement the same first capability and produce *different*
//! concrete successors carrying different evidence. That is the property a
//! hardcoded return type cannot express.
//!
//! The protocol here is one in-process pass. It ends at a persistable value and
//! deliberately does not claim a durable write, because a consuming Rust
//! transition moves a local value and does not consume a stored fact.

use std::error::Error;
use std::fmt;

// -------------------------------------------------------------------------
// Stage capabilities
// -------------------------------------------------------------------------

/// First capability: turn untrusted submission input into canonical values.
pub trait Canonicalize: Sized {
    /// Successor stage, bounded by the capability it must satisfy.
    type Next: CheckIdentity;

    /// Failure specific to canonicalization.
    type Error;

    /// Consumes the submission and produces its canonical successor.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the submission cannot be canonicalized.
    fn canonicalize(self) -> Result<Self::Next, Self::Error>;
}

/// Second capability: decide whether the canonical identity is available.
///
/// This stage branches, so it names one successor capability per outcome.
pub trait CheckIdentity: Sized {
    /// Successor when the identity is available.
    type Available: AcceptPolicy;

    /// Successor when an existing account already holds the identity.
    type Conflicting: ResolveConflict;

    /// Failure specific to the identity check.
    type Error;

    /// Consumes the canonical registration and reports which edge was taken.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when availability cannot be determined. An
    /// undetermined check is not a conflict and is not an approval.
    fn check_identity(
        self,
        directory: &IdentityDirectory,
    ) -> Result<IdentityOutcome<Self::Available, Self::Conflicting>, Self::Error>;
}

/// Third capability: record the policy version the applicant accepted.
pub trait AcceptPolicy: Sized {
    /// Successor stage, bounded by the capability it must satisfy.
    type Next: PreparePersistence;

    /// Failure specific to policy acceptance.
    type Error;

    /// Consumes the checked registration and records consent evidence.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the offered consent does not match the
    /// policy version currently in force.
    fn accept_policy(self, consent: OfferedConsent) -> Result<Self::Next, Self::Error>;
}

/// Fourth capability: assemble the value a durable writer may accept.
pub trait PreparePersistence: Sized {
    /// Terminal output of the in-process protocol.
    type Output;

    /// Failure specific to persistence preparation.
    type Error;

    /// Consumes the accepted registration and produces the persistable value.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when a required account identity is unavailable.
    fn prepare_persistence(self, account_id: AccountId) -> Result<Self::Output, Self::Error>;
}

/// Recovery capability reached only from the conflicting branch.
pub trait ResolveConflict: Sized {
    /// Successor when the applicant supplies a revised submission.
    ///
    /// The bound points back at [`Canonicalize`], so the retry edge re-enters
    /// the protocol at its first stage rather than skipping ahead.
    type Revised: Canonicalize;

    /// Failure specific to conflict resolution.
    type Error;

    /// Consumes the conflicting registration and reports the chosen recovery.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the revision is not a usable submission.
    fn resolve(
        self,
        revision: Option<RawSubmission>,
    ) -> Result<Recovery<Self::Revised>, Self::Error>;
}

// -------------------------------------------------------------------------
// Branch and recovery edges
// -------------------------------------------------------------------------

/// Named branch over the identity-check successors.
///
/// The alternatives are distinct types, so neither branch can be reached with
/// the other's evidence and neither carries optional fields standing in for a
/// state that was never established.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityOutcome<Available, Conflicting> {
    /// The canonical identity was not held by an existing account.
    Available(Available),
    /// An existing account already holds the canonical identity.
    Conflicting(Conflicting),
}

/// Named recovery edge from the conflicting branch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Recovery<Revised> {
    /// The applicant supplied a revised submission; the protocol restarts.
    Revised(Revised),
    /// The applicant abandoned the attempt; no further edge exists.
    Abandoned(AbandonedRegistration),
}

// -------------------------------------------------------------------------
// Canonical values
// -------------------------------------------------------------------------

/// Failure to establish a canonical value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueError {
    /// The address had no local part, no domain, or surrounding blank input.
    MalformedAddress,
    /// The display name was blank after trimming.
    BlankDisplayName,
}

impl fmt::Display for ValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedAddress => formatter.write_str("address is not in local@domain form"),
            Self::BlankDisplayName => formatter.write_str("display name is blank"),
        }
    }
}

impl Error for ValueError {}

/// Address normalized by this example's documented policy.
///
/// The policy is deliberately narrow: trim surrounding whitespace and lowercase
/// the domain. It establishes syntax and normalization, not mailbox ownership.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Parses and normalizes an address.
    ///
    /// # Errors
    ///
    /// Returns [`ValueError::MalformedAddress`] when the input is not a
    /// nonempty local part, one `@`, and a nonempty domain.
    pub fn parse(raw: &str) -> Result<Self, ValueError> {
        let trimmed = raw.trim();
        let (local, domain) = trimmed
            .split_once('@')
            .ok_or(ValueError::MalformedAddress)?;
        if local.is_empty() || domain.is_empty() || domain.contains('@') {
            return Err(ValueError::MalformedAddress);
        }
        Ok(Self(format!("{local}@{}", domain.to_lowercase())))
    }

    /// Returns the normalized address.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Display name trimmed to a nonempty value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayName(String);

impl DisplayName {
    /// Trims and rejects a blank display name.
    ///
    /// # Errors
    ///
    /// Returns [`ValueError::BlankDisplayName`] for blank input.
    pub fn parse(raw: &str) -> Result<Self, ValueError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(ValueError::BlankDisplayName);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the trimmed display name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Account identity supplied by the caller that owns identity allocation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AccountId(String);

impl AccountId {
    /// Constructs a nonempty account identity.
    ///
    /// # Errors
    ///
    /// Returns [`ValueError::BlankDisplayName`] when the identity is blank.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValueError::BlankDisplayName);
        }
        Ok(Self(value))
    }

    /// Returns the account identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Policy version identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolicyVersion(pub u32);

/// Consent as offered by the applicant, before it is checked.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfferedConsent {
    /// Policy version the applicant claims to have accepted.
    pub accepted_version: PolicyVersion,
}

/// Consent evidence produced by a successful policy check.
///
/// The field is private: the only way to obtain this value is to run the
/// policy stage, so possessing it is evidence that the check happened.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsentProof {
    version: PolicyVersion,
}

impl ConsentProof {
    /// Returns the policy version the check accepted.
    pub const fn version(self) -> PolicyVersion {
        self.version
    }
}

/// Evidence that no existing account held the canonical identity at check time.
///
/// The value is scoped to the observation that produced it. It does not prove
/// the identity is still free when a durable write is later attempted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniquenessObservation {
    observed: EmailAddress,
}

impl UniquenessObservation {
    /// Returns the address this observation covered.
    pub const fn observed(&self) -> &EmailAddress {
        &self.observed
    }
}

// -------------------------------------------------------------------------
// Origin evidence: what makes the two entry stages genuinely different
// -------------------------------------------------------------------------

/// Evidence carried by a self-service submission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfServiceOrigin {
    challenge_id: String,
}

impl SelfServiceOrigin {
    /// Returns the challenge that accompanied the submission.
    pub fn challenge_id(&self) -> &str {
        &self.challenge_id
    }
}

/// Evidence carried by an invited submission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvitedOrigin {
    invite_code: String,
    inviting_account: AccountId,
}

impl InvitedOrigin {
    /// Returns the invitation code.
    pub fn invite_code(&self) -> &str {
        &self.invite_code
    }

    /// Returns the account that issued the invitation.
    pub const fn inviting_account(&self) -> &AccountId {
        &self.inviting_account
    }
}

/// Runtime discriminant recorded when origin evidence is erased.
///
/// Erasure happens once, at the persistence boundary, and is named. It does not
/// happen between stages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OriginKind {
    /// The applicant submitted the registration directly.
    SelfService,
    /// The applicant used an invitation.
    Invited,
}

/// Origin evidence that can report its own runtime discriminant.
pub trait Origin {
    /// Returns the discriminant recorded at the persistence boundary.
    fn kind(&self) -> OriginKind;
}

impl Origin for SelfServiceOrigin {
    fn kind(&self) -> OriginKind {
        OriginKind::SelfService
    }
}

impl Origin for InvitedOrigin {
    fn kind(&self) -> OriginKind {
        OriginKind::Invited
    }
}

// -------------------------------------------------------------------------
// Stage types
// -------------------------------------------------------------------------

/// Untrusted submission input shared by both entry stages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawSubmission {
    /// Address exactly as received.
    pub address: String,
    /// Display name exactly as received.
    pub display_name: String,
}

/// Entry stage for a self-service submission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfServiceSubmission {
    /// Untrusted submission input.
    pub submission: RawSubmission,
    /// Challenge identifier presented with the submission.
    pub challenge_id: String,
}

/// Entry stage for an invited submission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvitedSubmission {
    /// Untrusted submission input.
    pub submission: RawSubmission,
    /// Invitation code presented with the submission.
    pub invite_code: String,
    /// Account that issued the invitation.
    pub inviting_account: AccountId,
}

/// Canonical registration carrying origin-specific evidence.
///
/// Fields are private and no public constructor exists, so this stage can be
/// reached only by running [`Canonicalize::canonicalize`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRegistration<O> {
    address: EmailAddress,
    display_name: DisplayName,
    origin: O,
}

impl<O> CanonicalRegistration<O> {
    /// Returns the canonical address.
    pub const fn address(&self) -> &EmailAddress {
        &self.address
    }

    /// Returns the canonical display name.
    pub const fn display_name(&self) -> &DisplayName {
        &self.display_name
    }

    /// Returns the origin evidence carried from the entry stage.
    pub const fn origin(&self) -> &O {
        &self.origin
    }
}

/// Registration whose canonical identity was observed to be available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AvailableRegistration<O> {
    registration: CanonicalRegistration<O>,
    uniqueness: UniquenessObservation,
}

impl<O> AvailableRegistration<O> {
    /// Returns the availability observation established by the check.
    pub const fn uniqueness(&self) -> &UniquenessObservation {
        &self.uniqueness
    }
}

/// Registration blocked by an existing account.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictingRegistration<O> {
    registration: CanonicalRegistration<O>,
    existing_account: AccountId,
}

impl<O> ConflictingRegistration<O> {
    /// Returns the account already holding the canonical identity.
    pub const fn existing_account(&self) -> &AccountId {
        &self.existing_account
    }
}

/// Registration whose consent evidence was established.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptedRegistration<O> {
    available: AvailableRegistration<O>,
    consent: ConsentProof,
}

impl<O> AcceptedRegistration<O> {
    /// Returns the consent evidence established by the policy stage.
    pub const fn consent(&self) -> ConsentProof {
        self.consent
    }
}

/// Terminal stage: the value a durable writer may accept.
///
/// Reaching this stage proves the in-process protocol ran in order. It proves
/// nothing about a stored row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistableRegistration {
    account_id: AccountId,
    address: EmailAddress,
    display_name: DisplayName,
    consent: ConsentProof,
    origin_kind: OriginKind,
}

impl PersistableRegistration {
    /// Returns the allocated account identity.
    pub const fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// Returns the canonical address.
    pub const fn address(&self) -> &EmailAddress {
        &self.address
    }

    /// Returns the canonical display name.
    pub const fn display_name(&self) -> &DisplayName {
        &self.display_name
    }

    /// Returns the consent evidence.
    pub const fn consent(&self) -> ConsentProof {
        self.consent
    }

    /// Returns the erased origin discriminant.
    pub const fn origin_kind(&self) -> OriginKind {
        self.origin_kind
    }
}

/// Terminal recovery stage with no further protocol edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbandonedRegistration {
    blocked_by: AccountId,
}

impl AbandonedRegistration {
    /// Returns the account that blocked the attempt.
    pub const fn blocked_by(&self) -> &AccountId {
        &self.blocked_by
    }
}

// -------------------------------------------------------------------------
// Stage-specific failures
// -------------------------------------------------------------------------

/// Failure of the canonicalization stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalizeError {
    /// Underlying value failure.
    pub cause: ValueError,
}

impl fmt::Display for CanonicalizeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "canonicalization failed: {}", self.cause)
    }
}

impl Error for CanonicalizeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

/// Failure of the identity-check stage.
///
/// An undetermined check is neither availability nor conflict. It is a third
/// outcome, and it keeps the address so an operator can look the attempt up.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityCheckError {
    /// Address whose availability could not be determined.
    pub undetermined_for: EmailAddress,
}

impl fmt::Display for IdentityCheckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "identity availability undetermined for {}",
            self.undetermined_for.as_str()
        )
    }
}

impl Error for IdentityCheckError {}

/// Failure of the policy stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolicyError {
    /// Version the applicant offered.
    pub offered: PolicyVersion,
    /// Version currently in force.
    pub required: PolicyVersion,
}

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "consent for policy version {} does not satisfy required version {}",
            self.offered.0, self.required.0
        )
    }
}

impl Error for PolicyError {}

/// Failure of the persistence-preparation stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreparationError {
    /// Underlying value failure.
    pub cause: ValueError,
}

impl fmt::Display for PreparationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "persistence preparation failed: {}", self.cause)
    }
}

impl Error for PreparationError {}

/// Failure of the conflict-resolution stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolutionError {
    /// Underlying value failure in the revised submission.
    pub cause: ValueError,
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "conflict resolution failed: {}", self.cause)
    }
}

impl Error for ResolutionError {}

// -------------------------------------------------------------------------
// Collaborators
// -------------------------------------------------------------------------

/// Read-only view of identities already taken, used by the check stage.
///
/// It is a plain in-memory list so the example stays deterministic and has no
/// network or database dependency.
#[derive(Clone, Debug, Default)]
pub struct IdentityDirectory {
    taken: Vec<(EmailAddress, AccountId)>,
    undetermined: Vec<EmailAddress>,
}

impl IdentityDirectory {
    /// Creates an empty directory.
    pub const fn new() -> Self {
        Self {
            taken: Vec::new(),
            undetermined: Vec::new(),
        }
    }

    /// Records that an address is already held by an account.
    #[must_use]
    pub fn with_taken(mut self, address: EmailAddress, holder: AccountId) -> Self {
        self.taken.push((address, holder));
        self
    }

    /// Records that an address cannot currently be resolved.
    #[must_use]
    pub fn with_undetermined(mut self, address: EmailAddress) -> Self {
        self.undetermined.push(address);
        self
    }

    fn lookup(&self, address: &EmailAddress) -> Result<Option<AccountId>, IdentityCheckError> {
        if self.undetermined.contains(address) {
            return Err(IdentityCheckError {
                undetermined_for: address.clone(),
            });
        }
        Ok(self
            .taken
            .iter()
            .find(|(taken, _)| taken == address)
            .map(|(_, holder)| holder.clone()))
    }
}

/// Policy version currently in force.
pub const REQUIRED_POLICY_VERSION: PolicyVersion = PolicyVersion(3);

// -------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------

fn canonical_parts(submission: &RawSubmission) -> Result<(EmailAddress, DisplayName), ValueError> {
    let address = EmailAddress::parse(&submission.address)?;
    let display_name = DisplayName::parse(&submission.display_name)?;
    Ok((address, display_name))
}

impl Canonicalize for SelfServiceSubmission {
    type Next = CanonicalRegistration<SelfServiceOrigin>;
    type Error = CanonicalizeError;

    fn canonicalize(self) -> Result<Self::Next, Self::Error> {
        let (address, display_name) =
            canonical_parts(&self.submission).map_err(|cause| CanonicalizeError { cause })?;
        Ok(CanonicalRegistration {
            address,
            display_name,
            origin: SelfServiceOrigin {
                challenge_id: self.challenge_id,
            },
        })
    }
}

impl Canonicalize for InvitedSubmission {
    type Next = CanonicalRegistration<InvitedOrigin>;
    type Error = CanonicalizeError;

    fn canonicalize(self) -> Result<Self::Next, Self::Error> {
        let (address, display_name) =
            canonical_parts(&self.submission).map_err(|cause| CanonicalizeError { cause })?;
        Ok(CanonicalRegistration {
            address,
            display_name,
            origin: InvitedOrigin {
                invite_code: self.invite_code,
                inviting_account: self.inviting_account,
            },
        })
    }
}

impl<O> CheckIdentity for CanonicalRegistration<O>
where
    O: Origin,
{
    type Available = AvailableRegistration<O>;
    type Conflicting = ConflictingRegistration<O>;
    type Error = IdentityCheckError;

    fn check_identity(
        self,
        directory: &IdentityDirectory,
    ) -> Result<IdentityOutcome<Self::Available, Self::Conflicting>, Self::Error> {
        if let Some(existing_account) = directory.lookup(&self.address)? {
            return Ok(IdentityOutcome::Conflicting(ConflictingRegistration {
                registration: self,
                existing_account,
            }));
        }
        let uniqueness = UniquenessObservation {
            observed: self.address.clone(),
        };
        Ok(IdentityOutcome::Available(AvailableRegistration {
            registration: self,
            uniqueness,
        }))
    }
}

impl<O> AcceptPolicy for AvailableRegistration<O>
where
    O: Origin,
{
    type Next = AcceptedRegistration<O>;
    type Error = PolicyError;

    fn accept_policy(self, consent: OfferedConsent) -> Result<Self::Next, Self::Error> {
        if consent.accepted_version != REQUIRED_POLICY_VERSION {
            return Err(PolicyError {
                offered: consent.accepted_version,
                required: REQUIRED_POLICY_VERSION,
            });
        }
        Ok(AcceptedRegistration {
            available: self,
            consent: ConsentProof {
                version: consent.accepted_version,
            },
        })
    }
}

impl<O> PreparePersistence for AcceptedRegistration<O>
where
    O: Origin,
{
    type Output = PersistableRegistration;
    type Error = PreparationError;

    fn prepare_persistence(self, account_id: AccountId) -> Result<Self::Output, Self::Error> {
        let registration = self.available.registration;
        Ok(PersistableRegistration {
            account_id,
            address: registration.address,
            display_name: registration.display_name,
            consent: self.consent,
            origin_kind: registration.origin.kind(),
        })
    }
}

impl<O> ResolveConflict for ConflictingRegistration<O>
where
    O: Origin,
{
    type Revised = SelfServiceSubmission;
    type Error = ResolutionError;

    fn resolve(
        self,
        revision: Option<RawSubmission>,
    ) -> Result<Recovery<Self::Revised>, Self::Error> {
        let Some(submission) = revision else {
            return Ok(Recovery::Abandoned(AbandonedRegistration {
                blocked_by: self.existing_account,
            }));
        };
        canonical_parts(&submission).map_err(|cause| ResolutionError { cause })?;
        Ok(Recovery::Revised(SelfServiceSubmission {
            submission,
            challenge_id: format!("revision-of-{}", self.existing_account.as_str()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AbandonedRegistration, AcceptPolicy, AcceptedRegistration, AccountId,
        AvailableRegistration, CanonicalRegistration, Canonicalize, CheckIdentity,
        ConflictingRegistration, DisplayName, EmailAddress, IdentityDirectory, IdentityOutcome,
        InvitedOrigin, InvitedSubmission, OfferedConsent, OriginKind, PersistableRegistration,
        PolicyVersion, PreparePersistence, REQUIRED_POLICY_VERSION, RawSubmission, Recovery,
        ResolveConflict, SelfServiceOrigin, SelfServiceSubmission, ValueError,
    };

    // ---------------------------------------------------------------------
    // Protocol topology evidence
    // ---------------------------------------------------------------------
    //
    // These assertions compile only while each stage's associated successor
    // type is exactly the documented one *and* still satisfies the successor
    // capability. Redirecting a `type Next`, widening a bound, or deleting an
    // implementation turns the documented graph into a compiler error instead
    // of silent drift.

    fn assert_canonicalize_edge<S, N>()
    where
        S: Canonicalize<Next = N>,
        N: CheckIdentity,
    {
    }

    fn assert_identity_edges<S, A, C>()
    where
        S: CheckIdentity<Available = A, Conflicting = C>,
        A: AcceptPolicy,
        C: ResolveConflict,
    {
    }

    fn assert_policy_edge<S, N>()
    where
        S: AcceptPolicy<Next = N>,
        N: PreparePersistence,
    {
    }

    fn assert_preparation_edge<S, O>()
    where
        S: PreparePersistence<Output = O>,
    {
    }

    fn assert_recovery_edge<S, R>()
    where
        S: ResolveConflict<Revised = R>,
        R: Canonicalize,
    {
    }

    #[test]
    fn stage_graph_matches_the_documented_topology() {
        assert_canonicalize_edge::<SelfServiceSubmission, CanonicalRegistration<SelfServiceOrigin>>(
        );
        assert_canonicalize_edge::<InvitedSubmission, CanonicalRegistration<InvitedOrigin>>();

        assert_identity_edges::<
            CanonicalRegistration<SelfServiceOrigin>,
            AvailableRegistration<SelfServiceOrigin>,
            ConflictingRegistration<SelfServiceOrigin>,
        >();
        assert_identity_edges::<
            CanonicalRegistration<InvitedOrigin>,
            AvailableRegistration<InvitedOrigin>,
            ConflictingRegistration<InvitedOrigin>,
        >();

        assert_policy_edge::<
            AvailableRegistration<SelfServiceOrigin>,
            AcceptedRegistration<SelfServiceOrigin>,
        >();
        assert_policy_edge::<
            AvailableRegistration<InvitedOrigin>,
            AcceptedRegistration<InvitedOrigin>,
        >();

        assert_preparation_edge::<AcceptedRegistration<SelfServiceOrigin>, PersistableRegistration>(
        );
        assert_preparation_edge::<AcceptedRegistration<InvitedOrigin>, PersistableRegistration>();

        assert_recovery_edge::<ConflictingRegistration<SelfServiceOrigin>, SelfServiceSubmission>();
        assert_recovery_edge::<ConflictingRegistration<InvitedOrigin>, SelfServiceSubmission>();
    }

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

    fn submission(address: &str) -> RawSubmission {
        RawSubmission {
            address: address.to_owned(),
            display_name: "  Example Applicant ".to_owned(),
        }
    }

    fn self_service(address: &str) -> SelfServiceSubmission {
        SelfServiceSubmission {
            submission: submission(address),
            challenge_id: "challenge-1".to_owned(),
        }
    }

    fn invited(address: &str) -> InvitedSubmission {
        InvitedSubmission {
            submission: submission(address),
            invite_code: "invite-1".to_owned(),
            inviting_account: AccountId::new("account-inviter").expect("nonblank identity"),
        }
    }

    fn accepted_consent() -> OfferedConsent {
        OfferedConsent {
            accepted_version: REQUIRED_POLICY_VERSION,
        }
    }

    // ---------------------------------------------------------------------
    // Narrative: the collapsed chain
    // ---------------------------------------------------------------------

    #[test]
    fn self_service_registration_runs_the_whole_protocol() {
        let directory = IdentityDirectory::new();
        let account_id = AccountId::new("account-1").expect("nonblank identity");

        let canonical = self_service("  Applicant@Example.COM ")
            .canonicalize()
            .expect("canonical values");
        assert_eq!(canonical.address().as_str(), "Applicant@example.com");
        assert_eq!(canonical.display_name().as_str(), "Example Applicant");
        assert_eq!(canonical.origin().challenge_id(), "challenge-1");

        let IdentityOutcome::Available(available) = canonical
            .check_identity(&directory)
            .expect("availability determined")
        else {
            panic!("empty directory must not report a conflict");
        };
        assert_eq!(
            available.uniqueness().observed().as_str(),
            "Applicant@example.com"
        );

        let persistable = available
            .accept_policy(accepted_consent())
            .expect("consent matches the version in force")
            .prepare_persistence(account_id.clone())
            .expect("persistable value");

        assert_eq!(persistable.account_id(), &account_id);
        assert_eq!(persistable.address().as_str(), "Applicant@example.com");
        assert_eq!(persistable.consent().version(), REQUIRED_POLICY_VERSION);
        assert_eq!(persistable.origin_kind(), OriginKind::SelfService);
    }

    #[test]
    fn invited_registration_carries_its_own_origin_evidence() {
        let directory = IdentityDirectory::new();
        let canonical = invited("applicant@example.com")
            .canonicalize()
            .expect("canonical values");

        assert_eq!(canonical.origin().invite_code(), "invite-1");
        assert_eq!(
            canonical.origin().inviting_account().as_str(),
            "account-inviter"
        );

        let IdentityOutcome::Available(available) = canonical
            .check_identity(&directory)
            .expect("availability determined")
        else {
            panic!("empty directory must not report a conflict");
        };

        let persistable = available
            .accept_policy(accepted_consent())
            .expect("consent matches")
            .prepare_persistence(AccountId::new("account-2").expect("nonblank identity"))
            .expect("persistable value");

        assert_eq!(persistable.origin_kind(), OriginKind::Invited);
    }

    // ---------------------------------------------------------------------
    // Branch, recovery, and failure edges
    // ---------------------------------------------------------------------

    #[test]
    fn taken_identity_takes_the_conflicting_branch() {
        let holder = AccountId::new("account-existing").expect("nonblank identity");
        let directory = IdentityDirectory::new().with_taken(
            EmailAddress::parse("applicant@example.com").expect("valid address"),
            holder.clone(),
        );

        let outcome = self_service("applicant@example.com")
            .canonicalize()
            .expect("canonical values")
            .check_identity(&directory)
            .expect("availability determined");

        let IdentityOutcome::Conflicting(conflicting) = outcome else {
            panic!("a taken address must take the conflicting branch");
        };
        assert_eq!(conflicting.existing_account(), &holder);
    }

    #[test]
    fn revision_reenters_the_protocol_at_the_first_stage() {
        let holder = AccountId::new("account-existing").expect("nonblank identity");
        let directory = IdentityDirectory::new().with_taken(
            EmailAddress::parse("applicant@example.com").expect("valid address"),
            holder,
        );

        let IdentityOutcome::Conflicting(conflicting) = self_service("applicant@example.com")
            .canonicalize()
            .expect("canonical values")
            .check_identity(&directory)
            .expect("availability determined")
        else {
            panic!("expected the conflicting branch");
        };

        let Recovery::Revised(revised) = conflicting
            .resolve(Some(submission("second@example.com")))
            .expect("revision is usable")
        else {
            panic!("a supplied revision must produce the revised edge");
        };

        let canonical = revised.canonicalize().expect("canonical values");
        assert_eq!(canonical.address().as_str(), "second@example.com");
    }

    #[test]
    fn abandoning_a_conflict_reaches_a_terminal_stage() {
        let holder = AccountId::new("account-existing").expect("nonblank identity");
        let directory = IdentityDirectory::new().with_taken(
            EmailAddress::parse("applicant@example.com").expect("valid address"),
            holder.clone(),
        );

        let IdentityOutcome::Conflicting(conflicting) = self_service("applicant@example.com")
            .canonicalize()
            .expect("canonical values")
            .check_identity(&directory)
            .expect("availability determined")
        else {
            panic!("expected the conflicting branch");
        };

        let Recovery::Abandoned(abandoned) = conflicting.resolve(None).expect("abandon is legal")
        else {
            panic!("no revision must produce the abandoned edge");
        };
        assert_eq!(abandoned, AbandonedRegistration { blocked_by: holder });
    }

    #[test]
    fn undetermined_availability_is_neither_branch() {
        let directory = IdentityDirectory::new().with_undetermined(
            EmailAddress::parse("applicant@example.com").expect("valid address"),
        );

        let error = self_service("applicant@example.com")
            .canonicalize()
            .expect("canonical values")
            .check_identity(&directory)
            .expect_err("an unresolvable directory must not report a branch");

        assert_eq!(error.undetermined_for.as_str(), "applicant@example.com");
    }

    #[test]
    fn stale_consent_fails_at_the_policy_stage() {
        let directory = IdentityDirectory::new();
        let IdentityOutcome::Available(available) = self_service("applicant@example.com")
            .canonicalize()
            .expect("canonical values")
            .check_identity(&directory)
            .expect("availability determined")
        else {
            panic!("expected the available branch");
        };

        let error = available
            .accept_policy(OfferedConsent {
                accepted_version: PolicyVersion(2),
            })
            .expect_err("a stale policy version must be rejected");

        assert_eq!(error.offered, PolicyVersion(2));
        assert_eq!(error.required, REQUIRED_POLICY_VERSION);
    }

    #[test]
    fn malformed_input_fails_at_the_canonicalization_stage() {
        let error = self_service("not-an-address")
            .canonicalize()
            .expect_err("a malformed address must be rejected");
        assert_eq!(error.cause, ValueError::MalformedAddress);

        let blank = SelfServiceSubmission {
            submission: RawSubmission {
                address: "applicant@example.com".to_owned(),
                display_name: "   ".to_owned(),
            },
            challenge_id: "challenge-1".to_owned(),
        };
        let error = blank
            .canonicalize()
            .expect_err("a blank display name must be rejected");
        assert_eq!(error.cause, ValueError::BlankDisplayName);
    }

    #[test]
    fn canonical_values_survive_every_transition() {
        let directory = IdentityDirectory::new();
        let persistable = self_service("  Applicant@Example.COM ")
            .canonicalize()
            .expect("canonical values")
            .check_identity(&directory)
            .expect("availability determined");

        let IdentityOutcome::Available(available) = persistable else {
            panic!("expected the available branch");
        };

        let prepared = available
            .accept_policy(accepted_consent())
            .expect("consent matches")
            .prepare_persistence(AccountId::new("account-3").expect("nonblank identity"))
            .expect("persistable value");

        // The raw submission strings are not carried past canonicalization.
        assert_eq!(prepared.address().as_str(), "Applicant@example.com");
        assert_eq!(
            prepared.display_name(),
            &DisplayName::parse("Example Applicant").expect("valid name")
        );
    }
}
