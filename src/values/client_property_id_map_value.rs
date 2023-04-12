use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::{uri::URI, uuid::UUID, *};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
enum ClientPropertyIDMapValueInner {
    URI(URI),
    UUID(UUID),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientPropertyIDMapValue {
    d1:    u8,
    inner: ClientPropertyIDMapValueInner,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClientPropertyIDMapValueError {
    OutOfRange,
}

impl ClientPropertyIDMapValue {
    pub fn from_u8_with_uri(
        d1: u8,
        uri: URI,
    ) -> Result<ClientPropertyIDMapValue, ClientPropertyIDMapValueError> {
        if d1 > 9 {
            return Err(ClientPropertyIDMapValueError::OutOfRange);
        }

        Ok(ClientPropertyIDMapValue {
            d1,
            inner: ClientPropertyIDMapValueInner::URI(uri),
        })
    }

    pub fn from_u8_with_uuid(
        d1: u8,
        uuid: UUID,
    ) -> Result<ClientPropertyIDMapValue, ClientPropertyIDMapValueError> {
        if d1 > 9 {
            return Err(ClientPropertyIDMapValueError::OutOfRange);
        }

        Ok(ClientPropertyIDMapValue {
            d1,
            inner: ClientPropertyIDMapValueInner::UUID(uuid),
        })
    }
}

impl ClientPropertyIDMapValue {
    pub fn get_uuid(&self) -> Option<&UUID> {
        if let ClientPropertyIDMapValueInner::UUID(uuid) = &self.inner {
            return Some(uuid);
        }

        None
    }

    pub fn get_uri(&self) -> Option<&URI> {
        if let ClientPropertyIDMapValueInner::URI(uri) = &self.inner {
            return Some(uri);
        }

        None
    }
}

impl Value for ClientPropertyIDMapValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{};", self.d1))?;

        match &self.inner {
            ClientPropertyIDMapValueInner::URI(uri) => {
                Value::fmt(uri, f)?;
            },
            ClientPropertyIDMapValueInner::UUID(uuid) => {
                Value::fmt(uuid, f)?;
            },
        }

        Ok(())
    }
}

impl Display for ClientPropertyIDMapValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for ClientPropertyIDMapValue {}

impl ValidatedWrapper for ClientPropertyIDMapValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
