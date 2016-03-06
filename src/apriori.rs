
// std lib uses
//use std::collections::BTreeMap;
use std::collections::{ BTreeMap, BTreeSet};
use std::fs::File;
use std::sync::mpsc::{ Receiver, sync_channel};
use std::thread;

// local uses
use util::BasketFileReader;

pub struct ItemSetMiner {
	buffer_size: usize,
	reader: BasketFileReader,
	support: f32,
}

impl ItemSetMiner {
	pub fn new( file: File, buffer_size: usize, support: f32) -> ItemSetMiner {
		ItemSetMiner {
			buffer_size: buffer_size,
			reader: BasketFileReader::new( file),
			support: support,}}

	pub fn pass_one(
			basket_rx : &Receiver<Vec<u32>>,
			term_rx : &Receiver<u32>,
			support: f32) -> ( u32, f32, BTreeSet<u32>) {

		//vars
		let mut map : BTreeMap<u32,u32> = BTreeMap::new();
		let mut i = 0;
		let mut term_result = term_rx.try_recv();

		// count item occurence
		loop {
			// check stop condition
			if term_result.is_err() {
				term_result = term_rx.try_recv()}
			if let Ok( count) = term_result {
				if i >= count { break;}}

			// increment for each item
			if let Ok( basket) = basket_rx.try_recv() {
				i += 1;
				for x in basket {
					let mut new = 1;
					if let Some( &n) = map.get( &x) {
						new = n + 1;}
					map.insert( x, new);}}}

		// find items above support
		let n = term_result.unwrap();
		let s = ( (n as f32) * support).floor();
		let mut result = BTreeSet::new();
		result.extend(
			map.iter()
			.filter( | &( &_, &c) | c as f32 > s)
			.map( | ( &x, &_) | x));

		// clean up and return
		( n, s, result)}

	pub fn pass_two(
			basket_rx: &Receiver<Vec<u32>>,
			term_rx: &Receiver<u32>,
			candidates: Vec<(u32, u32)>,
			s: f32) -> BTreeSet<(u32, u32)> {

		//vars
		let mut map : BTreeMap<(u32, u32),u32> = BTreeMap::new();
		let mut i = 0;
		let mut term_result = term_rx.try_recv();

		// count pair occurence
		loop {
			// check stop condition
			if term_result.is_err() {
				term_result = term_rx.try_recv()}
			if let Ok( count) = term_result {
				if i >= count { break;}}

			// for each basket
			if let Ok( basket) = basket_rx.try_recv() {
				i += 1;

				// check for pair presence
				for &(x, y) in candidates.iter() {
					if basket.contains( &x) && basket.contains( &y) {
						let mut new = 1;
						if let Some( &n) = map.get( &(x, y)) {
							new = n + 1;}
						map.insert( (x, y), new);}}}}

		// find pairs above support
		let mut result = BTreeSet::new();
		result.extend(
			map.iter()
			.filter( | &( &_, &c) | c as f32 > s)
			.map( | ( &x, &_) | x));

		// clean up and return
		result}

	pub fn run( &mut self) -> BTreeSet<(u32, u32)> {
		let ( basket_tx, basket_rx) =
			sync_channel::<Vec<u32>>( self.buffer_size);
		let ( term_tx, term_rx) =
			sync_channel::<u32>( 0);
		let support = self.support;

		// run pass one
		// start processing
		let handle = thread::spawn( move || {
			ItemSetMiner::pass_one(
				&basket_rx, &term_rx, support)});
		// start reading
		let count = self.reader.run( &basket_tx);
		// send termination signal
		term_tx.send( count).ok();
		// get result
		let ( n, s, freq_items) = handle.join().unwrap();
		println!( "pass one complete");

		// debug output
		println!( "n: {}", n);
		println!( "s: {}", s);
		println!( "freq_items: {:?}", freq_items);

		// construct candidates
		let mut candidates : Vec<(u32, u32)> = Vec::new();
		for &x in freq_items.iter() {
			for &y in freq_items.iter() {
				if x < y {
					candidates.push( (x, y));}}}
		println!( "candidates: {:?}", candidates);

		// make new channels
		let ( basket_tx, basket_rx) =
			sync_channel::<Vec<u32>>( self.buffer_size);
		let ( term_tx, term_rx) =
			sync_channel::<u32>( 0);

		// run pass two
		// start processing
		let handle = thread::spawn( move || {
			ItemSetMiner::pass_two(
				&basket_rx, &term_rx, candidates, s)});
		// start reading
		let count = self.reader.run( &basket_tx);
		// send termination signal
		term_tx.send( count).ok();
		// get result
		let freq_pairs = handle.join().unwrap();
		println!( "pass two complete");

		// debug output
		println!( "freq_pairs: {:?}", freq_pairs);

		// return
		freq_pairs}
}