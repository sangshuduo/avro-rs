use std::env;
use avrow::Header;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Hello, {}!", &args[1]);

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(&args[1]).unwrap();

    let header = Header::from_reader(&mut file).unwrap();
    println!("{}", header.schema());
}

