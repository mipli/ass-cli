use clap::{value_t, values_t, ArgMatches};
use std::io::Write;
use termcolor::Buffer;
use termcolor::{Color, ColorSpec, WriteColor};

use crate::AssCliError;
use ass_rs::Account;

pub fn handle(
    account: &Account,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    match matches.subcommand() {
        ("upload", Some(matches)) => handle_upload(account, matches, buffer, verbose),
        ("search", Some(matches)) => handle_search(account, matches, buffer, verbose),
        _ => Err(AssCliError::command_error()),
    }
}

fn handle_search(
    account: &Account,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let path = value_t!(matches, "path", String)?;
    let queries = vec![("path", &path[..])];
    let files = account.search_files(&queries)?;
    for file in &files {
        if verbose {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(buffer, "\nFile found: ")?;
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            writeln!(buffer, "{}", file)?;
        }

        let url = account.get_file_url(file.get_path().ok_or(AssCliError::json_error())?)?;
        write_url(&url, buffer)?;
    }
    Ok(())
}

fn handle_upload(
    account: &Account,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let mut destination = value_t!(matches, "destination", String)?;
    match destination.chars().last() {
        Some('/') => {}
        _ => destination.push_str("/"),
    };
    let cache_time = value_t!(matches, "cache", u32)?;
    let files = values_t!(matches.values_of("files"), String)?;
    for file in &files {
        let data = account.upload_file_with_headers(
            file,
            &destination,
            &[("Cache-Control", &format!("max-age: {}", cache_time))],
        )?;

        if verbose {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(buffer, "\nFile uploaded: ")?;
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            writeln!(buffer, "{}", data)?;
        }

        let url = account.get_file_url(data.get_path().ok_or(AssCliError::json_error())?)?;
        write_url(&url, buffer)?;
    }
    Ok(())
}

fn write_url(url: &str, buffer: &mut Buffer) -> Result<(), AssCliError> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", url)?;
    Ok(())
}
