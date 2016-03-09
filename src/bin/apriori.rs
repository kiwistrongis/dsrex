// library imports
extern crate dsrex;

// std lib uses
use std::env;
use std::fs::File;

// dsrex uses
use dsrex::apriori::ItemSetMiner;

fn main(){
	// default arg values
	let mut buffer = 16;
	let mut support = 0.01;
	let mut filename = "data/retail.dat".to_string();

	// parse args
	let mut args = env::args().skip( 1);
	while let Some( arg) = args.next() {
		if arg == "-b" {
			let next_arg = &*args.next().unwrap();
			buffer = next_arg.parse().unwrap();}
		else if arg == "-s" {
			let next_arg = &*args.next().unwrap();
			support = next_arg.parse().unwrap();}
		else {
			filename = arg;
			break;}}

	// run algorithm
	let file = File::open( filename).unwrap();
	let mut miner = ItemSetMiner::new( file, buffer, support);
	let result_map = miner.run();

	// print top 100 pairs over support threshold
	let mut pairs : Vec<((u32, u32), u32)> = Vec::new();
	pairs.extend( result_map.iter().map(
		| (&(x, y), &count) | ((x, y), count)));
	pairs.sort_by(
		| &( _, a_count), &( _, b_count)| a_count.cmp( &b_count));
	for &( pair, count) in pairs.iter().rev().take( 100) {
		println!( "{:?}, {}", pair, count);}
}