extern crate native_tls;
use self::native_tls::TlsStream;
use std::io::{self, Read, Write};

pub enum Stream<S>
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
