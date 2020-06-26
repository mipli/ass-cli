use crate::AssCliError;
use clap::{value_t, ArgMatches};
use std::io::Write;
use termcolor::Buffer;

use ass_rs::AssClient;

pub fn handle(
    ass_client: &AssClient,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    _verbose: bool,
) -> Result<(), AssCliError> {
    let url = value_t!(matches, "url", String)?;
    let signed = ass_client.sign_url(&url)?;
    writeln!(buffer, "{:?}", signed)?;
    Ok(())
}
