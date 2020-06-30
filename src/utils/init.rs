extern crate clap;
extern crate toml;
extern crate curl;

use clap::{App, Arg};
use curl::easy::Easy;
use std::fs;
use std::io::{BufReader, Read};
use std::io::{stdout, Write};

pub fn parse_args() -> (Vec<String>, bool, bool, bool) {
    let app = App::new("zoo-scanner")
        .version("1.0")
        .about("Simple Web Scanner")
        .arg(Arg::with_name("targets")
            .help("target URLs")
            .short("t")
            .long("target")
            .required(true)
            .takes_value(true)
            .multiple(true)
        )
        .arg(Arg::with_name("xss")
            .help("do xss scan")
            .short("x")
            .long("xss")
        )
        .arg(Arg::with_name("directoryt")
            .help("do directory traversal scan")
            .short("d")
            .long("directoryt")
        )
        .arg(Arg::with_name("osci")
            .help("do os command injection scan[")
            .short("o")
            .long("osci")
        );
            
    let matches = app.get_matches();
    let urls: Vec<String> = matches.values_of("targets").unwrap().
        map(|x|x.to_string()).collect();
    let enable_xss: bool = matches.is_present("xss");
    let enable_directoryt: bool = matches.is_present("directoryt");
    let enable_osci: bool = matches.is_present("osci");

    (urls, enable_xss, enable_directoryt, enable_osci)
}

#[derive(Debug,Deserialize)]
struct Conf {
    pathcheck: Option<PathCheck>,
}

impl Conf {
    fn get_paths(self) -> Vec<String> {
        self.pathcheck.unwrap().targets.unwrap()
    }
}

#[derive(Debug,Deserialize)]
struct PathCheck {
    targets: Option<Vec<String>>,
}

fn read_file(target_file: &str) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(target_file)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_config(config_path: &str) -> Vec<String> {
    let file_content = match read_file(config_path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read config: {}", e),
    };
    let conf_wrapped: Result<Conf, toml::de::Error> = toml::from_str(&file_content);
    let conf = match conf_wrapped {
        Ok(p) => p,
        Err(e) => panic!("Failed to parse config: {}", e),
    };
    conf.get_paths()
}

pub fn init_easy() -> Easy {
    let mut easy = Easy::new();
    easy.verbose(true).unwrap();
    easy.write_function(
        |data| Ok(stdout().write(data).unwrap())
    ).unwrap();
    easy
}
