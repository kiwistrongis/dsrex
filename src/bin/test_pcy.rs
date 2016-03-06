// library imports
extern crate dsrex;

// std lib uses
use std::env;
use std::fs::File;
use std::str::FromStr;

// dsrex uses
use dsrex::pcy::ItemSetMiner;

fn main(){
	// default arg values
	let mut buffer = 16;
	let mut support = 0.1;
	let mut filename = "data/retail.dat".to_string();

	// parse args
	let mut args = env::args().skip( 1);
	while let Some( arg) = args.next() {
		if arg == "-b" {
			buffer = usize::from_str( &*args.next().unwrap()).unwrap();}
		else if arg == "-s" {
			support = f32::from_str( &*args.next().unwrap()).unwrap();}
		else {
			filename = arg;
			break;}}

	// run algorithm
	let file = File::open( filename).unwrap();
	let mut miner = ItemSetMiner::new( file, buffer, support);
	miner.run();
}