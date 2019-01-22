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
        ("upload", Some(matches)) => handle_upload(account, matches, buffer),
        _ => unreachable!()
    }
}

fn get_file_url(path: &str, account: &Account) -> Result<Url, Error> {
    let url = Url::parse(&account.url)?;
    let url = url.join(&format!("users/{}/files/{}", account.name, path))?;
    account.sign_url(url)
}

fn handle_upload(account: &Account, matches: &ArgMatches, buffer: &mut Buffer) -> Result<(), Error>  {
    let mut destination = value_t!(matches, "destination", String)?;
    match destination.chars().last() {
        Some('/') => {},
        _ => destination.push_str("/")
    };
    let files = values_t!(matches.values_of("files"), String)?;
    for file in &files {
        let data = upload(file.into(), &destination, account)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(buffer, "\nFile uploaded: ")?;
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        writeln!(buffer, "{}", data)?;

        let url = get_file_url(data.get_path().ok_or(Error::JsonError)?, account)?;
        write_url(&url, buffer)?;

    }
    Ok(())
}

fn write_url(url: &Url, buffer: &mut Buffer) -> Result<(), Error> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(buffer, "URL: ")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(buffer, "{}", url)?;
    Ok(())
}

fn upload(path: PathBuf, destination: &str, account: &Account) -> Result<AssData, Error> {
    let url = Url::parse(&account.url)?;
    let url = url.join(&format!("files/{}", destination))?;
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let url = url.join(file_name)?;

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
