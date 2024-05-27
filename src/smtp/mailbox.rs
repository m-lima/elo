#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Address(#[from] lettre::address::AddressError),
    #[error("Missing name for mailbox")]
    MissingName,
}

#[derive(Clone)]
pub struct Mailbox(lettre::message::Mailbox);

impl Mailbox {
    pub fn new(name: String, email: String) -> Result<Self, Error> {
        macro_rules! trim {
            ($value: expr) => {{
                let trimmed = $value.trim();
                if trimmed != $value {
                    String::from(trimmed)
                } else {
                    $value
                }
            }};
        }

        let name = trim!(name);
        if name.is_empty() {
            return Err(Error::MissingName);
        }

        let email = trim!(email);

        let address = email.parse()?;

        Ok(Self(lettre::message::Mailbox::new(Some(name), address)))
    }

    #[must_use]
    pub fn name(&self) -> &str {
        // SAFETY: This was already checked during construction
        unsafe { self.0.name.as_ref().unwrap_unchecked().as_ref() }
    }

    #[must_use]
    pub fn email(&self) -> &str {
        self.0.email.as_ref()
    }
}

impl std::str::FromStr for Mailbox {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim_start()
            .parse()
            .map(Self)
            .map_err(Error::from)
            .and_then(|a| {
                if a.0.name.is_none() {
                    Err(Error::MissingName)
                } else {
                    Ok(a)
                }
            })
    }
}

impl From<Mailbox> for lettre::message::Mailbox {
    fn from(value: Mailbox) -> Self {
        value.0
    }
}

impl std::fmt::Display for Mailbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for Mailbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn white_spaces_are_invalid_for_mailbox() {
        let err = <lettre::message::Mailbox as std::str::FromStr>::from_str(
            "   Name Is Here    <     email@domain.com    >   ",
        )
        .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidInput);

        let err = <lettre::message::Mailbox as std::str::FromStr>::from_str(
            "   Name Is Here    <     email@domain.com    >",
        )
        .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidInput);

        let err = <lettre::message::Mailbox as std::str::FromStr>::from_str(
            "   Name Is Here    <     email@domain.com>    ",
        )
        .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidUser);

        let name =
            <lettre::message::Mailbox as std::str::FromStr>::from_str("   <email@domain.com>    ")
                .unwrap()
                .name;
        assert_eq!(name, None);

        let name = <lettre::message::Mailbox as std::str::FromStr>::from_str(
            "   	   <email@domain.com>    ",
        )
        .unwrap()
        .name;
        assert_eq!(name, None);
    }

    #[test]
    fn white_spaces_are_invalid_for_address() {
        let err = <lettre::address::Address as std::str::FromStr>::from_str("").unwrap_err();
        assert_eq!(err, lettre::address::AddressError::MissingParts);

        let err = <lettre::address::Address as std::str::FromStr>::from_str(
            "        email@domain.com       ",
        )
        .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidUser);

        let err =
            <lettre::address::Address as std::str::FromStr>::from_str("        email@domain.com")
                .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidUser);

        let err =
            <lettre::address::Address as std::str::FromStr>::from_str("email@domain.com       ")
                .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidDomain);

        let err = <lettre::address::Address as std::str::FromStr>::from_str(
            "   Name Is Here    <     email@domain.com>    ",
        )
        .unwrap_err();
        assert_eq!(err, lettre::address::AddressError::InvalidUser);
    }

    #[test]
    fn trim_prefix_is_enough() {
        let mailbox: super::Mailbox = "   Name    <email@domain.com>    ".parse().unwrap();
        assert_eq!(mailbox.name(), "Name");
        assert_eq!(mailbox.email(), "email@domain.com");
    }
}
