extern crate home;

use std::env;
use std::fs;
use std::fs::File;
use std::process::Command;
use std::process::exit;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Write;

fn main() {
	//Collects the arguments that will be passed.
	let args: Vec<String> = env::args().collect();
	
	//If there isn't any arguments (rust starts from 1)
	if args.len() == 1 {
		exit(1);
	} 
	
	//Get the first argument.
	let appimage = &args[1];
	
	//If the path/file doesn't exist
	if !(Path::new(appimage).exists()) {
		exit(1);
	}
	
	//Reverse the entire path then split it.
										    //Reverse part
	let name: Vec<String> = appimage.chars().rev().collect::<String>()
							//Split part
							.split("/").map(|s| s.to_string()).collect();

	//Get the first part of the split then reverse it back.
	let appimagename = &name[0].chars().rev().collect::<String>();
	
	//Get the home directory of the user
	let homedir = format!("{}", home::home_dir().unwrap().display()); 

	//Create directories both for applications and pixmaps
	fs::create_dir_all(format!("{}/.local/share/applications", homedir)).ok();
	fs::create_dir_all(format!("{}/.local/share/pixmaps", homedir)).ok();
	
	//Execute command for extracting appimages
	Command::new(appimage)
	 .arg("--appimage-extract")
	 .output()
	 .expect("Failed to execute");
	
	//Read everything in the extracted appimage
	let sqfs = fs::read_dir("squashfs-root/").unwrap();
	
	//Loop for the files
	for files in sqfs {
		
		//Make the filename friendly to use
		let fi = format!("{}", files.unwrap().path().display());
		
		//If the file ends with .desktop, if not check .png, if not check .svg
		if fi.to_lowercase().ends_with(".desktop") {
			
			//used for to replace the old Exec= part, getting ready to read lines
			let replacewith = format!("Exec={}", appimage);
			
			//Open the file
			let f = File::open(fi).expect("failed to open file");
			
			//Open a stream reader
			let f = BufReader::new(f);
			
			//Where the .desktop will be
			let path = format!("{}/.local/share/apps/{}.desktop", homedir, appimagename);

			//Create .desktop file
			let mut output = File::create(path).expect("failed to create file");

			//Loop through the original .desktop file
			for line in f.lines() {

				//Define line
				let mut line = line.expect("failed to read line");
				
				//If file starts with Exec=
				if line.starts_with("Exec=") {
					
					//Get the executable that will be replaced
					let replace: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

					//Then replace it
					line = line.replace(&replace[0], &replacewith);
				}
				
				//Write it onto the new .desktop file
				writeln!(output, "{}", line).expect("Failed to write");
			}
		}
		else if fi.to_lowercase().ends_with(".png"){
			//Split the path
			let filename: Vec<String> = fi.split("/").map(|s| s.to_string()).collect();
			
			//Then copy the 2nd part, which has the image file.
			fs::copy(fi, format!("{}/.local/share/pixmaps/{}", homedir, &filename[1])).ok();
		}
		//.svg is the same thing
		else if fi.to_lowercase().ends_with(".svg") {
			let filename: Vec<String> = fi.split("/").map(|s| s.to_string()).collect();
			fs::copy(fi, format!("{}/.local/share/pixmaps/{}", homedir, &filename[1])).ok();
		}
	}

	//Remove the squashfs-root/ directory.
	fs::remove_dir_all("squashfs-root/").ok();
}
