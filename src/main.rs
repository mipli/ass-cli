use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};
use termcolor::{BufferWriter, ColorChoice};
use ass_rs::{Account};
use failure::{Error};
use std::path::{PathBuf};

mod error;
mod image;
mod file;

use error::{AssCliError};

fn main() -> Result<(), Error> {
    let config_dir = dirs::config_dir().ok_or(AssCliError::PathError)?;
    let config_dir_string = config_dir.to_str().ok_or(AssCliError::PathError)?;
    let matches = App::new("ASS (Aptoma Smooth Storage) CLI tool")
                          .version("0.1")
                          .author("Michael Plikk <michael@plikk.com>")
                          .about("Tool to ease interaction with ASS from the CLI")
                          .setting(AppSettings::ArgRequiredElseHelp)
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .takes_value(true)
                               .conflicts_with("account")
                               .help("Path to config file with account information"))
                          .arg(Arg::with_name("account")
                               .short("a")
                               .long("account")
                               .takes_value(true)
                               .conflicts_with("config")
                               .help(&format!("Account name, using account file in {}/ass-cli/<account>.conf", config_dir_string)))
                          .subcommand(SubCommand::with_name("image")
                                      .about("Operate on ASS images")
                                      .subcommand(SubCommand::with_name("data")
                                                  .about("get data about image")
                                                  .arg(Arg::with_name("id")
                                                      .index(1)
                                                      .required(true)
                                                      .help("Image id to get")))
                                      .subcommand(SubCommand::with_name("url")
                                                  .about("get signed url for image")
                                                  .arg(Arg::with_name("id")
                                                      .index(1)
                                                      .required(true)
                                                      .help("Image id to get")))
                                      .subcommand(SubCommand::with_name("upload")
                                                  .about("upload image from path")
                                                  .arg(Arg::with_name("files")
                                                      .required(true)
                                                      .min_values(1)
                                                      .help("image path")))
                          )
                          .subcommand(SubCommand::with_name("file")
                                      .about("Operate on ASS files")
                                      .subcommand(SubCommand::with_name("upload")
                                                  .about("upload file from path")
                                                  .arg(Arg::with_name("destination")
                                                      .short("d")
                                                      .long("destination")
                                                      .takes_value(true)
                                                      .help("destination folder on server"))
                                                  .arg(Arg::with_name("files")
                                                      .required(true)
                                                      .min_values(1)
                                                      .help("file path")))
                          )
                          .get_matches();

    let account = get_account(&config_dir, &matches)?;

    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    match matches.subcommand() {
        ("image", Some(matches)) => image::handle(&account, matches, &mut buffer)?,
        ("file", Some(matches)) => file::handle(&account, matches, &mut buffer)?,
        _ => {}
    }

    bufwtr.print(&buffer)?;

    Ok(())
}

fn get_account(config_dir: &PathBuf, matches: &ArgMatches) -> Result<Account, AssCliError> {
    let account = if let Some(acc) = matches.value_of("account") {
        let path = config_dir.join(format!("ass-cli/{}.json", acc));
        Account::from_file(path)?
    } else {
        let config = matches.value_of("config").unwrap_or("account.json");
        Account::from_file(&config)?
    };
    Ok(account)
}
