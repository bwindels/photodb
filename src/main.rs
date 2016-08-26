mod metadata;

use std::env;
use std::io::Read;
use std::fs::File;

fn main() {
	let filename = env::args().nth(1).expect("could not open file");
	let mut file = File::open(filename).unwrap();
	let mut buffer = [0; 1024 * 100];
	let bytes_read = file.read(&mut buffer).unwrap();
	let read_buffer = &buffer[0 .. bytes_read];
	let md = metadata::parse_metadata(read_buffer);
	println!("metadata: {:?}", md);
}

