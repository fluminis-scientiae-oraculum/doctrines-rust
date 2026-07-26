//! Checked conversion from Serde DTOs and persistence rows.

use serde::Deserialize;
use std::error::Error;
use std::fmt;

const MAX_EMAIL_BYTES: usize = 254;
const MAX_NAME_CHARS: usize = 80;

/// Domain conversion failure shared by request and row adapters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContactError {
    /// Email violates the example syntax policy.
    InvalidEmail,
    /// Display name is empty, too long, or contains controls.
    InvalidDisplayName,
    /// Persistence status tag is unsupported.
    UnsupportedStatus,
}

impl fmt::Display for ContactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidEmail => "contact email is invalid",
            Self::InvalidDisplayName => "contact display name is invalid",
            Self::UnsupportedStatus => "contact status is unsupported",
        };
        formatter.write_str(message)
    }
}

impl Error for ContactError {}

/// Syntax-validated contact email.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactEmail(String);

impl ContactEmail {
    fn parse(value: &str) -> Result<Self, ContactError> {
        let value = value.trim();
        if value.is_empty() || value.len() > MAX_EMAIL_BYTES || value.chars().any(char::is_control)
        {
            return Err(ContactError::InvalidEmail);
        }

        let (local, domain) = value.split_once('@').ok_or(ContactError::InvalidEmail)?;
        if local.is_empty()
            || domain.contains('@')
            || !domain.contains('.')
            || domain.split('.').any(str::is_empty)
        {
            return Err(ContactError::InvalidEmail);
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the normalized address.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Bounded nonempty display name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayName(String);

impl DisplayName {
    fn parse(value: &str) -> Result<Self, ContactError> {
        let value = value.trim();
        if value.is_empty()
            || value.chars().count() > MAX_NAME_CHARS
            || value.chars().any(char::is_control)
        {
            return Err(ContactError::InvalidDisplayName);
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the normalized display name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize)]
struct RawContactDto {
    email: String,
    display_name: String,
}

/// Trusted contact constructed through a Serde `try_from` path.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(try_from = "RawContactDto")]
pub struct Contact {
    email: ContactEmail,
    display_name: DisplayName,
}

impl Contact {
    /// Returns the validated email.
    pub const fn email(&self) -> &ContactEmail {
        &self.email
    }

    /// Returns the validated display name.
    pub const fn display_name(&self) -> &DisplayName {
        &self.display_name
    }
}

impl TryFrom<RawContactDto> for Contact {
    type Error = ContactError;

    fn try_from(raw: RawContactDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: ContactEmail::parse(&raw.email)?,
            display_name: DisplayName::parse(&raw.display_name)?,
        })
    }
}

/// Raw persistence representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactRow {
    /// Physical email column.
    pub email: String,
    /// Physical display-name column.
    pub display_name: String,
    /// Stable persisted status tag.
    pub status: String,
}

impl TryFrom<ContactRow> for Contact {
    type Error = ContactError;

    fn try_from(row: ContactRow) -> Result<Self, Self::Error> {
        if row.status != "active" {
            return Err(ContactError::UnsupportedStatus);
        }
        Ok(Self {
            email: ContactEmail::parse(&row.email)?,
            display_name: DisplayName::parse(&row.display_name)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Contact, ContactError, ContactRow};

    #[test]
    fn serde_uses_checked_domain_conversion() {
        let contact: Contact =
            serde_json::from_str(r#"{"email":"user@example.com","display_name":"Ada"}"#)
                .expect("valid DTO constructs contact");

        assert_eq!(contact.email().as_str(), "user@example.com");
        assert_eq!(contact.display_name().as_str(), "Ada");
    }

    #[test]
    fn serde_rejects_invalid_trusted_value() {
        let error =
            serde_json::from_str::<Contact>(r#"{"email":"not-an-address","display_name":"Ada"}"#)
                .expect_err("invalid email must not construct contact");

        assert!(error.to_string().contains("contact email is invalid"));
    }

    #[test]
    fn invalid_historical_row_is_rejected() {
        let row = ContactRow {
            email: "historical-invalid".to_owned(),
            display_name: "Ada".to_owned(),
            status: "active".to_owned(),
        };

        assert_eq!(Contact::try_from(row), Err(ContactError::InvalidEmail));
    }

    #[test]
    fn unknown_persisted_status_is_rejected() {
        let row = ContactRow {
            email: "user@example.com".to_owned(),
            display_name: "Ada".to_owned(),
            status: "future-state".to_owned(),
        };

        assert_eq!(Contact::try_from(row), Err(ContactError::UnsupportedStatus));
    }
}
