use clap::{value_t, values_t, ArgMatches};
use std::io::Write;
use termcolor::Buffer;
use termcolor::{Color, ColorSpec, WriteColor};

use crate::AssCliError;
use ass_rs::{image_handling, AssClient};

pub async fn handle(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    match matches.subcommand() {
        ("data", Some(matches)) => get_data(ass_client, matches, buffer).await,
        ("url", Some(matches)) => handle_url(ass_client, matches, buffer),
        ("upload", Some(matches)) => handle_upload(ass_client, matches, buffer, verbose).await,
        _ => Err(AssCliError::command_error()),
    }
}

async fn handle_upload(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
    verbose: bool,
) -> Result<(), AssCliError> {
    let files = values_t!(matches.values_of("files"), String)?;

    for file in &files {
        let data = image_handling::upload_image(ass_client, file).await?;

        if verbose {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(buffer, "\nImage uploaded: ")?;
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            writeln!(buffer, "{}", data)?;
        }

        let url = image_handling::get_image_url(ass_client, data.id)?;
        write_url(&url, buffer)?;
    }

    Ok(())
}

async fn get_data(
    ass_client: &AssClient,
    matches: &ArgMatches<'static>,
    buffer: &mut Buffer,
) -> Result<(), AssCliError> {
    let image_id = value_t!(matches, "id", u64)?;
    let data = image_handling::get_image_information(ass_client, image_id).await?;

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "\nOutput: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", data)?;

    let url = image_handling::get_image_url(ass_client, image_id)?;
    write_url(&url, buffer)?;

    Ok(())
}

fn handle_url(
    ass_client: &AssClient,
    matches: &ArgMatches,
    buffer: &mut Buffer,
) -> Result<(), AssCliError> {
    let image_id = value_t!(matches, "id", u64)?;

    let url = image_handling::get_image_url(ass_client, image_id)?;
    writeln!(buffer, "\n")?;
    write_url(&url, buffer)?;
    Ok(())
}

fn write_url(url: &str, buffer: &mut Buffer) -> Result<(), AssCliError> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", url)?;
    Ok(())
}
