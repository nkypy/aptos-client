use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A string encoded U64
///
/// Encoded as a string to encode into JSON
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64(pub u64);

impl From<u64> for U64 {
    fn from(d: u64) -> Self {
        Self(d)
    }
}

impl From<U64> for u64 {
    fn from(d: U64) -> Self {
        d.0
    }
}

impl fmt::Display for U64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Serialize for U64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for U64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        Ok(Self(s.parse::<u64>().map_err(D::Error::custom)?))
    }
}
