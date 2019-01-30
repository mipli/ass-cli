use clap::{ArgMatches, value_t, values_t};
use std::io::Write;
use termcolor::{Buffer};
use termcolor::{Color, ColorSpec, WriteColor};
use failure::{Error};

use ass_rs::{Account};
use crate::{AssCliError};

pub fn handle(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error> {
    match matches.subcommand() {
        ("upload", Some(matches)) => handle_upload(account, matches, buffer),
        _ => unreachable!()
    }
}

fn handle_upload(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let mut destination = value_t!(matches, "destination", String)?;
    match destination.chars().last() {
        Some('/') => {},
        _ => destination.push_str("/")
    };
    let files = values_t!(matches.values_of("files"), String)?;
    for file in &files {
        let data = account.upload_file(file, &destination)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(buffer, "\nFile uploaded: ")?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        writeln!(buffer, "{}", data)?;

        let url = account.get_file_url(data.get_path().ok_or(AssCliError::JsonError)?)?;
        write_url(&url, buffer)?;

    }
    Ok(())
}

fn write_url(url: &str, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", url)?;
    Ok(())
}
