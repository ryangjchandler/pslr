use std::path::Path;

use anyhow::Result;
use publicsuffix::{List, Psl};

#[derive(Debug)]
pub struct PublicSuffixList {
    inner: List,
}

impl PublicSuffixList {
    pub fn suffix<'a>(&self, name: &'a [u8]) -> Option<&'a [u8]> {
        self.inner.suffix(name).map(|s| s.as_bytes())
    }

    pub fn domain<'a>(&self, name: &'a [u8]) -> Option<&'a [u8]> {
        self.inner.domain(name).map(|s| s.as_bytes())
    }
}

impl TryFrom<&Path> for PublicSuffixList {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let contents = std::fs::read_to_string(value)?;
        let inner = List::from_bytes(contents.as_bytes()).map_err(|e| anyhow::anyhow!(e))?;

        Ok(Self { inner })
    }
}

impl TryFrom<&[u8]> for PublicSuffixList {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let inner = List::from_bytes(value).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Self { inner })
    }
}
