use ass_rs::AssClient;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
pub use error::{AssCliError, AssCliErrorKind};
use std::path::PathBuf;
use termcolor::{BufferWriter, ColorChoice};

mod error;
mod file;
mod image;
mod sign;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> Result<(), AssCliError> {
    let config_dir = dirs::config_dir().ok_or_else(AssCliError::path_error)?;
    let config_dir_string = config_dir.to_str().ok_or_else(AssCliError::path_error)?;
    let matches = App::new("ASS (Aptoma Smooth Storage) CLI tool")
        .version("1.0")
        .author("Michael Plikk <michael@plikk.com>")
        .about("Tool to ease interaction with ASS from the CLI")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .conflicts_with("account")
                .help("Path to config file with account information"),
        )
        .arg(
            Arg::with_name("account")
                .short("a")
                .long("account")
                .takes_value(true)
                .conflicts_with("config")
                .help(&format!(
                    "Account name, using account file in {}/ass-cli/<account>.conf",
                    config_dir_string
                )),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose output"),
        )
        .subcommand(
            SubCommand::with_name("image")
                .about("Operate on ASS images")
                .subcommand(
                    SubCommand::with_name("info")
                        .about("get information about image")
                        .arg(
                            Arg::with_name("id")
                                .index(1)
                                .required(true)
                                .help("Image id to get"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("url")
                        .about("get signed url for image")
                        .arg(
                            Arg::with_name("id")
                                .index(1)
                                .required(true)
                                .help("Image id to get"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("upload")
                        .about("upload image from path")
                        .arg(
                            Arg::with_name("files")
                                .required(true)
                                .min_values(1)
                                .help("image path"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("file")
                .about("Operate on ASS files")
                .subcommand(
                    SubCommand::with_name("upload")
                        .about("upload file from path")
                        .arg(
                            Arg::with_name("destination")
                                .short("d")
                                .long("destination")
                                .default_value("/")
                                .takes_value(true)
                                .help("destination folder on server"),
                        )
                        .arg(
                            Arg::with_name("cache")
                                .short("c")
                                .long("cache")
                                .default_value("31557600")
                                .takes_value(true)
                                .help("cache time for uploaded file, in seconds"),
                        )
                        .arg(
                            Arg::with_name("files")
                                .required(true)
                                .min_values(1)
                                .help("file path"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("search")
                        .about("search for files")
                        .arg(Arg::with_name("path").required(true).help(
                            "file path to search for. Path is exact match, use '%' as wildcard",
                        )),
                )
                .subcommand(
                    SubCommand::with_name("info")
                        .about("Get basic information about file")
                        .arg(Arg::with_name("key").required(true).help("File id or path")),
                )
                .subcommand(
                    SubCommand::with_name("render")
                        .about("Render preview of file")
                        .arg(Arg::with_name("key").required(true).help("File id or path")),
                ),
        )
        .subcommand(
            SubCommand::with_name("sign").about("Sign a Url").arg(
                Arg::with_name("url")
                    .index(1)
                    .required(true)
                    .help("url to sign"),
            ),
        )
        .get_matches();

    let ass_client = get_ass_client(&config_dir, &matches)?;

    let verbose = matches.is_present("verbose");

    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    match matches.subcommand() {
        ("image", Some(matches)) => {
            image::handle(&ass_client, matches, &mut buffer, verbose).await?
        }
        ("file", Some(matches)) => file::handle(&ass_client, matches, &mut buffer, verbose).await?,
        ("sign", Some(matches)) => sign::handle(&ass_client, matches, &mut buffer, verbose)?,
        _ => {}
    }

    bufwtr.print(&buffer)?;

    Ok(())
}

fn get_ass_client(config_dir: &PathBuf, matches: &ArgMatches) -> Result<AssClient, AssCliError> {
    let ass_client = if let Some(acc) = matches.value_of("account") {
        let path = config_dir.join(format!("ass-cli/{}.json", acc));
        AssClient::from_file(&path).map_err(|_| {
            let path = match path.to_str() {
                Some(p) => p.to_string(),
                None => "Unknown path".to_string(),
            };
            AssCliError::invalid_account_file(path)
        })?
    } else {
        let config = matches.value_of("config").unwrap_or("account.json");
        AssClient::from_file(&config)
            .map_err(|_| AssCliError::invalid_account_file(config.to_string()))?
    };
    Ok(ass_client)
}
