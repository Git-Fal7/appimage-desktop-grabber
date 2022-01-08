use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
	let args: Vec<String> = env::args().collect();
	
	let file = &args[1];
	let replace = &args[2];
	let replacewith = &args[3];
	let f = File::open(file).expect("failed to open file");
	let f = BufReader::new(f);
	
	for line in f.lines() {
		let mut line = line.expect("failed to read line");
		if line.contains(replace) {
			let nline = line.replace(replace, replacewith);
			line = nline;
		}
		println!("{}", line);
	}
}
