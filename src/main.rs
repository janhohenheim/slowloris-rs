extern crate slowloris;

fn main() {
    let url = std::env::args()
        .nth(1)
        .expect("Please pass an url to the program");
    slowloris::do_loris(&url).unwrap();
    println!("Hello, world!");
}
