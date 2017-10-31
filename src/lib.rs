#![feature(conservative_impl_trait)]
extern crate url;
extern crate native_tls;
use url::{Url, Host};

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;


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
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));

}


fn get_stream(url: &Url) -> Box<ReadWrite> {
    let domain = url.host_str().unwrap();
    let port = url.port_or_known_default().unwrap();
    let stream = TcpStream::connect((domain, port)).unwrap();
    if url.scheme() == "https" {
        let connector = TlsConnector::builder().unwrap().build().unwrap();
        Box::new(connector.connect(domain, stream).unwrap())
    } else {
        Box::new(stream)
    }
}

trait ReadWrite: Read + Write {}
impl<T> ReadWrite for T
where
    T: Read + Write,
{
}
