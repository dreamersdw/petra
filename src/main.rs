use actix_web::{fs::StaticFiles, server, App};
use clap::{self, Arg, ArgMatches};
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process;

struct Config {
    port: u16,
    dir: PathBuf,
}

#[derive(Debug)]
enum AppError {
    ArgError(String),
    IoError(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            AppError::ArgError(reason) => write!(f, "arg parse error, {}", reason),
            AppError::IoError(error) => write!(f, "{}", error),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> AppError {
        AppError::ArgError(error.to_string())
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> AppError {
        AppError::IoError(error)
    }
}

fn parse_matches(matches: &ArgMatches) -> Result<Config, AppError> {
    let port: u16 = matches
        .value_of("port")
        .ok_or_else(|| AppError::ArgError("missing arg: 'port'".to_owned()))?
        .parse()?;

    let dir: PathBuf = PathBuf::from(
        matches
            .value_of("dir")
            .ok_or_else(|| AppError::ArgError("missing arg 'dir'".to_owned()))?,
    );
    Ok(Config { port, dir })
}

fn run_server() -> Result<(), AppError> {
    let matches = clap::App::new("petra")
        .version("0.1.0")
        .author("dreamersdw")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .value_name("DIR")
                .default_value(".")
                .help("the directory contains markdown pages"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .default_value("3000")
                .help("the port peta will listen on"),
        )
        .get_matches();

    let config = parse_matches(&matches)?;
    let port = config.port;
    let dir = config.dir.clone();

    let address = format!("127.0.0.1:{}", port);
    let app = move || {
        let file_handler = StaticFiles::new(dir.clone()).unwrap().show_files_listing();
        App::new().handler("/", file_handler)
    };

    let server = server::new(app).bind(&address)?;
    println!("start petra server on http://{}", &address);
    server.run();
    Ok(())
}

fn main() {
    if let Err(e) = run_server() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
