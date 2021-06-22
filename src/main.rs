use std::env;
use avrow::Header;

fn main() {
    let args: Vec<String> = env::args().collect();

    let result = std::fs::OpenOptions::new()
        .read(true)
        .open(&args[1]);

    match result {
        Ok(mut file) => {
            let header = Header::from_reader(&mut file).unwrap();
            println!("{}", header.schema());
        }

        Err(_e) => {
            println!("Error on processing {}!", &args[1]);
        }
    }
}

