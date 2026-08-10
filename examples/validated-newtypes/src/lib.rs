//! Private validated values and evidence-accurate email verification.
//!
//! The email parser is intentionally an example syntax policy, not a complete
//! RFC implementation and not evidence of mailbox ownership or deliverability.

use std::error::Error;
use std::fmt;

const MAX_EMAIL_BYTES: usize = 254;
const MAX_LOCAL_BYTES: usize = 64;
const MAX_NAME_CHARS: usize = 80;

/// Syntax-policy rejection for an email address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmailError {
    /// Input is empty after surrounding whitespace normalization.
    Empty,
    /// Input exceeds the example byte limit.
    TooLong,
    /// Input includes a control character.
    ControlCharacter,
    /// Input does not contain exactly one `@` separator.
    Separator,
    /// Local part is empty or too long.
    LocalPart,
    /// Domain does not satisfy the example ASCII label policy.
    Domain,
}

impl fmt::Display for EmailError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "email input is empty",
            Self::TooLong => "email input is too long",
            Self::ControlCharacter => "email input contains a control character",
            Self::Separator => "email input must contain exactly one @",
            Self::LocalPart => "email local part is invalid",
            Self::Domain => "email domain is invalid",
        };
        formatter.write_str(message)
    }
}

impl Error for EmailError {}

/// Address accepted by this crate's syntax policy.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Parses and normalizes a syntax-level email address.
    ///
    /// # Errors
    ///
    /// Returns [`EmailError`] when input violates the documented example
    /// policy.
    pub fn parse(value: impl Into<String>) -> Result<Self, EmailError> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(EmailError::Empty);
        }
        if value.len() > MAX_EMAIL_BYTES {
            return Err(EmailError::TooLong);
        }
        if value.chars().any(char::is_control) {
            return Err(EmailError::ControlCharacter);
        }

        let (local, domain) = value.split_once('@').ok_or(EmailError::Separator)?;
        if domain.contains('@') {
            return Err(EmailError::Separator);
        }
        if local.is_empty() || local.len() > MAX_LOCAL_BYTES {
            return Err(EmailError::LocalPart);
        }
        if !valid_domain(domain) {
            return Err(EmailError::Domain);
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the normalized syntax-level address.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for EmailAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("EmailAddress")
            .field(&"<redacted>")
            .finish()
    }
}

fn valid_domain(domain: &str) -> bool {
    if !domain.contains('.') {
        return false;
    }

    domain.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    })
}

/// Failure to construct a bounded nonempty name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NameError {
    /// Input is empty after trimming.
    Empty,
    /// Input exceeds the character bound.
    TooLong,
    /// Input contains control characters.
    ControlCharacter,
}

impl fmt::Display for NameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "name is empty",
            Self::TooLong => "name is too long",
            Self::ControlCharacter => "name contains a control character",
        };
        formatter.write_str(message)
    }
}

impl Error for NameError {}

/// Nonempty display name bounded by Unicode scalar count.
#[derive(Clone, Eq, PartialEq)]
pub struct BoundedName(String);

impl BoundedName {
    /// Returns the normalized name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BoundedName {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(NameError::Empty);
        }
        if value.chars().count() > MAX_NAME_CHARS {
            return Err(NameError::TooLong);
        }
        if value.chars().any(char::is_control) {
            return Err(NameError::ControlCharacter);
        }
        Ok(Self(value.to_owned()))
    }
}

impl fmt::Debug for BoundedName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedName")
            .field("chars", &self.0.chars().count())
            .finish()
    }
}

/// Evidence emitted by a trusted ownership-verification adapter.
///
/// No public constructor exists. In a real service, the adapter constructs
/// this only after an authenticated challenge result.
#[derive(Debug)]
pub struct OwnershipProof {
    address: EmailAddress,
    verification_id: String,
}

impl OwnershipProof {
    #[cfg(test)]
    const fn recorded_by_provider(address: EmailAddress, verification_id: String) -> Self {
        Self {
            address,
            verification_id,
        }
    }
}

/// Module-owned verifier that consumes provider proof.
#[derive(Debug, Default)]
pub struct EmailVerifier;

impl EmailVerifier {
    /// Converts verifier-owned evidence into a stronger email type.
    ///
    /// `OwnershipProof` deliberately has no public constructor in this
    /// standalone crate. A production provider adapter in this crate would
    /// construct it only after authenticating external verification evidence;
    /// unit tests exercise that construction topology without claiming a
    /// provider integration exists here.
    pub fn accept(proof: OwnershipProof) -> VerifiedEmailAddress {
        VerifiedEmailAddress {
            address: proof.address,
            verification_id: proof.verification_id,
        }
    }
}

/// Address that passed the ownership-verification adapter represented here.
///
/// This type does not prove future deliverability or continued mailbox control.
#[derive(Eq, PartialEq)]
pub struct VerifiedEmailAddress {
    address: EmailAddress,
    verification_id: String,
}

impl VerifiedEmailAddress {
    /// Returns the syntax-level address associated with the proof.
    pub const fn address(&self) -> &EmailAddress {
        &self.address
    }

    /// Returns the verification event identity.
    pub fn verification_id(&self) -> &str {
        &self.verification_id
    }
}

impl fmt::Debug for VerifiedEmailAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedEmailAddress")
            .field("address", &"<redacted>")
            .field("verification_id", &self.verification_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundedName, EmailAddress, EmailError, EmailVerifier, NameError, OwnershipProof};

    #[test]
    fn email_rejects_weak_separator_check_cases() {
        assert_eq!(
            EmailAddress::parse("missing-domain@"),
            Err(EmailError::Domain)
        );
        assert_eq!(
            EmailAddress::parse("two@@example.com"),
            Err(EmailError::Separator)
        );
        assert_eq!(
            EmailAddress::parse("user@localhost"),
            Err(EmailError::Domain)
        );
    }

    #[test]
    fn email_normalizes_surrounding_whitespace() {
        let address =
            EmailAddress::parse("  user@example.com  ").expect("example policy accepts address");
        assert_eq!(address.as_str(), "user@example.com");
        assert_eq!(format!("{address:?}"), "EmailAddress(\"<redacted>\")");
    }

    #[test]
    fn bounded_name_rejects_empty_and_long_values() {
        assert_eq!(
            BoundedName::try_from("  ".to_owned()),
            Err(NameError::Empty)
        );
        assert_eq!(
            BoundedName::try_from("x".repeat(81)),
            Err(NameError::TooLong)
        );
    }

    #[test]
    fn only_verifier_consumes_ownership_proof() {
        let address = EmailAddress::parse("owner@example.com").expect("valid example address");
        let proof =
            OwnershipProof::recorded_by_provider(address.clone(), "verification-7".to_owned());
        let verified = EmailVerifier::accept(proof);

        assert_eq!(verified.address(), &address);
        assert_eq!(verified.verification_id(), "verification-7");
    }
}
