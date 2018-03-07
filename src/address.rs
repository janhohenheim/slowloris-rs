use url::{ParseError, Url};

use err::LorisError;

use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub struct Address {
    addr: Url,
}

impl Address {
    pub fn path(&self) -> &str {
        self.addr.path()
    }

    pub fn host(&self) -> &str {
        self.addr.host_str().expect("Failed to read host")
    }

    pub fn port_or_known_default(&self) -> u16 {
        self.addr
            .port_or_known_default()
            .expect("Failed to guess port for specified protocol")
    }

    pub fn is_https(&self) -> bool {
        self.addr.scheme() == "https"
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.addr.fmt(f)
    }
}

impl FromStr for Address {
    type Err = LorisError;
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        let parsed_addr = addr.parse::<Url>();
        let parsed_addr = if let Err(ParseError::RelativeUrlWithoutBase) = parsed_addr {
            // No scheme was specified, let's try a default
            const DEFAULT_SCHEME: &str = "http://";
            let with_scheme = DEFAULT_SCHEME.to_string() + addr;
            with_scheme.parse::<Url>()?
        } else {
            parsed_addr?
        };
        Ok(Address { addr: parsed_addr })
    }
}
