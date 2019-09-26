use clap::{ArgMatches, value_t};
use std::io::Write;
use termcolor::{Buffer};
use crate::{AssCliError};

use ass_rs::{Account};

pub fn handle(account: &Account, matches: &ArgMatches, buffer: &mut Buffer, _verbose: bool) -> Result<(), AssCliError> {
    let url = value_t!(matches, "url", String)?;
    let signed = account.sign_url(&url)?;
    writeln!(buffer, "{:?}", signed)?;
    Ok(())
}
