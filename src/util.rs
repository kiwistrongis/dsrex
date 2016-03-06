
// std lib uses
use std::fs::File;
use std::io::prelude::*;
use std::io::{ SeekFrom, BufReader};
use std::str::FromStr;
use std::sync::mpsc::SyncSender;
use std::time::Duration;
use std::thread::sleep;

pub struct BasketFileReader {
	reader: BufReader<File>,
}

impl BasketFileReader {
	pub fn new( file: File) -> BasketFileReader {
		BasketFileReader {
			reader: BufReader::new( file)}}

	pub fn run( &mut self, out: &SyncSender< Vec<u32>>) -> u32 {
		// start reading the file from the beginning
		self.reader.seek( SeekFrom::Start( 0)).ok();
		let mut count = 0;

		// read each line
		let mut line = String::new();
		loop {
			let read_result = self.reader.read_line( &mut line);
			if read_result.is_err() { break;}
			if let Ok( len) = read_result {
				if len == 0 { break;}}

			// extract basket
			let mut basket : Vec<u32> = Vec::new();
			for word in line.split_whitespace() {
				if let Ok( item) = u32::from_str( word) {
					basket.push( item);}
				else { continue;}}

			// clean up and send basket
			line.clear();
			let send_result = out.send( basket);
			count += 1;
			if send_result.is_err() { break;}}
		count}
}

pub fn sleep_nanos( ns: u32){
	sleep( Duration::new( 0, ns))}

pub fn sleep_millis( ms: u32){
	sleep( Duration::new( 0, ms * 1000))}
