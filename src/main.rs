extern crate slowloris;

fn main() {
    slowloris::do_loris("https://www.google.com");
    println!("Hello, world!");
}
