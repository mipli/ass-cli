use clap::{ArgMatches, value_t, values_t};
use reqwest::{Url};
use reqwest::multipart::Form;
use std::io::Write;
use std::path::PathBuf;
use crate::{Account, Error};
use termcolor::{Buffer};
use termcolor::{Color, ColorSpec, WriteColor};

use crate::data::{AssData};

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
        let data = upload(file.into(), account)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(buffer, "\nImage uploaded: ")?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        writeln!(buffer, "{}", data)?;

        let url = get_image_url(data.get_id().ok_or(Error::JsonError)?, account)?;
        write_url(&url, buffer)?;
    }

    Ok(())
}

fn upload(path: PathBuf, account: &Account) -> Result<AssData, Error> {
    let url = Url::parse(&account.url)?;
    let url = url.join("images")?;

    let form = Form::new().file("file", path)?;

    let client = reqwest::Client::builder()
        .default_headers(account.get_headers()?)
        .build()?;

    let mut res = client
        .post(url)
        .multipart(form)
        .send()?;
    let data: AssData = res.text()?.parse()?;
    Ok(data)
}

fn get_data(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let image_id = value_t!(matches, "id", u64)?;

    let url = Url::parse(&account.url)?;
    let url = url.join(&format!("images/{}", image_id))?;

    let client = reqwest::Client::builder()
        .default_headers(account.get_headers()?)
        .build()?;
    let mut res = client.get(url).send()?;
    let data: AssData = res.text()?.parse()?;

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "\nOutput: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", data)?;

    let url = get_image_url(image_id, account)?;
    write_url(&url, buffer)?;

    Ok(())
}

fn handle_url(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let image_id = value_t!(matches, "id", u64)?;

    let url = get_image_url(image_id, account)?;
    writeln!(buffer, "\n")?;
    write_url(&url, buffer)?;
    Ok(())
}

fn write_url(url: &Url, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", url)?;
    Ok(())
}

fn get_image_url(id: u64, account: &Account) -> Result<Url, Error> {
    let url = Url::parse(&account.url)?;
    let url = url.join(&format!("users/{}/images/{}.jpg", account.name, id))?;
    account.sign_url(url)
}
