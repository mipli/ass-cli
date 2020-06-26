use clap::{value_t, values_t, ArgMatches};
use std::io::Write;
use termcolor::Buffer;
use termcolor::{Color, ColorSpec, WriteColor};

use crate::AssCliError;
use ass_rs::{file_handling, image_handling, AssClient};

pub fn handle(
    ass_client: &AssClient,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    match matches.subcommand() {
        ("upload", Some(matches)) => handle_upload(ass_client, matches, buffer, verbose),
        ("search", Some(matches)) => handle_search(ass_client, matches, buffer, verbose),
        ("info", Some(matches)) => handle_info(ass_client, matches, buffer, verbose),
        ("render", Some(matches)) => handle_render(ass_client, matches, buffer, verbose),
        _ => Err(AssCliError::command_error()),
    }
}

fn handle_search(
    ass_client: &AssClient,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let path = value_t!(matches, "path", String)?;
    let queries = vec![("path", &path[..])];
    let files = file_handling::search(ass_client, &queries)?;
    for file in &files {
        if verbose {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(buffer, "\nFile found: ")?;
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            writeln!(buffer, "{}", file)?;
        }

        let url = file_handling::get_file_url(ass_client, &file.path)?;
        write_url(&url, buffer)?;
    }
    Ok(())
}

fn handle_upload(
    ass_client: &AssClient,
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
        let data = file_handling::upload_file_with_headers(
            ass_client,
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

        let url = file_handling::get_file_url(ass_client, &data.path)?;
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

fn handle_info(
    ass_client: &AssClient,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let path = value_t!(matches, "path", String)?;
    let information = file_handling::get_file_information(ass_client, &path)?;
    if verbose {
        writeln!(buffer, "Raw file information: {:#?}", information)?;
    }

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "ID: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", information.id)?;

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(
        buffer,
        "{}",
        file_handling::get_file_url(ass_client, &information.path)?
    )?;
    Ok(())
}

fn handle_render(
    ass_client: &AssClient,
    matches: &ArgMatches,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let file_id = value_t!(matches, "id", u64)?;
    let information = file_handling::get_file_rendition(ass_client, file_id)?;
    let image_id = information.id;
    if verbose {
        writeln!(buffer, "Rendered iamge: {:#?}", information)?;
    }
    let url = image_handling::get_image_url(ass_client, image_id)?;
    write_url(&url, buffer)?;
    Ok(())
}
