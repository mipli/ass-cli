use std::fmt::{Display, Debug};
use std::str::FromStr;
use serde_json::{Value};

use crate::{Error};

pub struct AssData {
    inner: Value
}

impl AssData {
    pub fn get_id(&self) -> Option<u64> {
        self.inner.get("id")?.as_u64()
    }

    pub fn get_path(&self) -> Option<&str> {
        self.inner.get("path")?.as_str()
    }
}

impl FromStr for AssData {
    type Err = Error;
    fn from_str(s: &str) -> Result<AssData, Error> {
        let data: Value = serde_json::from_str(s)?;
        Ok(AssData {
            inner: data
        })
    }
}

impl Display for AssData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl Debug for AssData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s = serde_json::to_string_pretty(&self.inner).unwrap_or_else(|_| "Invalid JSON".to_string());
        write!(fmt, "{}", s)
    }
}
