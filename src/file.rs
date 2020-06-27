use clap::{value_t, values_t, ArgMatches};
use std::io::Write;
use termcolor::Buffer;
use termcolor::{Color, ColorSpec, WriteColor};

use crate::AssCliError;
use ass_rs::{file_handling, image_handling, AssClient};

pub async fn handle(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    match matches.subcommand() {
        ("upload", Some(matches)) => handle_upload(ass_client, matches, buffer, verbose).await,
        ("search", Some(matches)) => handle_search(ass_client, matches, buffer, verbose).await,
        ("info", Some(matches)) => handle_info(ass_client, matches, buffer, verbose).await,
        ("render", Some(matches)) => handle_render(ass_client, matches, buffer, verbose).await,
        _ => Err(AssCliError::command_error()),
    }
}

async fn handle_search(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let path = value_t!(matches, "path", String)?;
    let queries = vec![("path", &path[..])];
    let files = file_handling::search(ass_client, &queries).await?;
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

async fn handle_upload(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
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
        )
        .await?;

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

async fn handle_info(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let information = match value_t!(matches, "key", u64) {
        Ok(file_id) => file_handling::get_file_information_by_id(ass_client, file_id).await?,
        Err(_) => {
            let key = value_t!(matches, "key", String)?;
            file_handling::get_file_information(ass_client, &key).await?
        }
    };
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

async fn handle_render(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let image_data = match value_t!(matches, "key", u64) {
        Ok(file_id) => file_handling::get_file_rendition(ass_client, file_id).await?,
        Err(_) => {
            let key = value_t!(matches, "key", String)?;
            let file_information = file_handling::get_file_information(ass_client, &key).await?;
            file_handling::get_file_rendition(ass_client, file_information.id).await?
        }
    };
    if verbose {
        writeln!(buffer, "Raw iamge data: {:#?}", image_data)?;
    }
    let url = image_handling::get_image_url(ass_client, image_data.id)?;
    write_url(&url, buffer)?;
    Ok(())
}
