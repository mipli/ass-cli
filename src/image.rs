use clap::{ArgMatches, value_t, values_t};
use std::io::Write;
use termcolor::{Buffer};
use termcolor::{Color, ColorSpec, WriteColor};
use failure::{Error};

use ass_rs::{Account};
use crate::{AssCliError};

pub fn handle(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error> {
    match matches.subcommand() {
        ("data", Some(matches)) => get_data(account, matches, buffer),
        ("url", Some(matches)) => handle_url(account, matches, buffer),
        ("upload", Some(matches)) => handle_upload(account, matches, buffer),
        _ => unreachable!()
    }
}

fn handle_upload(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let files = values_t!(matches.values_of("files"), String)?;

    for file in &files {
        let data = account.upload_image(file)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(buffer, "\nImage uploaded: ")?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        writeln!(buffer, "{}", data)?;

        let url = account.get_image_url(data.get_id().ok_or(AssCliError::JsonError)?)?;
        write_url(&url, buffer)?;
    }

    Ok(())
}

fn get_data(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let image_id = value_t!(matches, "id", u64)?;
    let data = account.get_image_data(image_id)?;

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "\nOutput: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", data)?;

    let url = account.get_image_url(image_id)?;
    write_url(&url, buffer)?;

    Ok(())
}

fn handle_url(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let image_id = value_t!(matches, "id", u64)?;

    let url = account.get_image_url(image_id)?;
    writeln!(buffer, "\n")?;
    write_url(&url, buffer)?;
    Ok(())
}

fn write_url(url: &str, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", url)?;
    Ok(())
}
