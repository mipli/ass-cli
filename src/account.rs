use crate::{Error};
use reqwest::{Url};
use ring::{digest, hmac};
use std::path::PathBuf;
use reqwest::header::{HeaderMap};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub url: String,
    pub name: String,
    pub apikey: String
}

impl Account {
    pub fn get_headers(&self) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("bearer {}", self.apikey).parse()?);
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("x-ass-acl", "public".parse()?);

        Ok(headers)
    }

    pub fn sign_url(&self, url: Url) -> Result<Url, Error> {
        let key = hmac::SigningKey::new(&digest::SHA256, &self.apikey.as_bytes());
        let signature = hmac::sign(&key, &url.as_str().as_bytes());
        let s: String = signature.as_ref().iter().map(|s| format!("{:02x}", s)).collect();
        Url::parse_with_params(url.as_str(), &[("accessToken", &s)])
            .map_err(|err| err.into())
    }

    pub fn from_file<T: Into<PathBuf>>(file: T) -> Result<Self, Error> {
        let path = file.into();
        let mut file = File::open(&path)
            .map_err(|err| Error::NotFound(err.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| Error::PermissionDenied(err.to_string()))?;
        serde_json::from_str::<Account>(&contents)
            .map_err(|err| Error::InvalidAccountFile(err.to_string()))
    }

}
