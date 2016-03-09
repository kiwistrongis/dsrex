
// std lib uses
//use std::collections::BTreeMap;
use std::collections::{ BTreeMap, BTreeSet};
use std::fs::File;
use std::sync::mpsc::{ Receiver, sync_channel};
use std::thread;

// local uses
use util::{ BasketFileReader, hash};

pub struct ItemSetMiner {
	pub debug: bool,
	buffer_size: usize,
	reader: BasketFileReader,
	support: f32,
}

impl ItemSetMiner {
	pub fn new( file: File, buffer_size: usize, support: f32) -> ItemSetMiner {
		ItemSetMiner {
			debug: false,
			buffer_size: buffer_size,
			reader: BasketFileReader::new( file),
			support: support,}}

	pub fn pass_one(
			basket_rx : &Receiver<Vec<u32>>,
			term_rx : &Receiver<u32>,
			support: f32)
			-> ( u32, f32, BTreeSet<u32>, BTreeMap<u16,u32>) {

		//vars
		let mut item_map : BTreeMap<u32,u32> = BTreeMap::new();
		let mut bucket_map : BTreeMap<u16,u32> = BTreeMap::new();
		let mut i = 0;
		let mut term_result = term_rx.try_recv();

		// count item occurence
		loop {
			// check stop condition
			if term_result.is_err() {
				term_result = term_rx.try_recv()}
			if let Ok( count) = term_result {
				if i >= count { break;}}

			// for each basket
			if let Ok( basket) = basket_rx.try_recv() {
				i += 1;
				for &x in basket.iter() {
					// increment for each item
					let mut new = 1;
					if let Some( &n) = item_map.get( &x) {
						new = n + 1;}
					item_map.insert( x, new);}

				for &x in basket.iter() {
					// increment for each pair's bucket
					for &y in basket.iter() {
						if y <= x { continue;}
						let hash = hash( &( x, y)) as u16;
						let mut new = 1;
						if let Some( &n) = bucket_map.get( &hash) {
							new = n + 1;}
						bucket_map.insert( hash, new);}}}}

		// find items above support
		let n = term_result.unwrap();
		let s = ( (n as f32) * support).floor();
		let mut result = BTreeSet::new();
		result.extend(
			item_map.iter()
			.filter( | &( &_, &c) | c as f32 > s)
			.map( | ( &x, &_) | x));

		// clean up and return
		( n, s, result, bucket_map)}

	pub fn pass_two(
			basket_rx: &Receiver<Vec<u32>>,
			term_rx: &Receiver<u32>,
			candidates: Vec<(u32, u32)>,
			support: f32) -> BTreeMap<(u32, u32), u32> {

		//vars
		let mut pair_map : BTreeMap<(u32, u32),u32> = BTreeMap::new();
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
				for &x in basket.iter() {
					for &y in basket.iter() {
						if y <= x { continue;}
						if candidates.contains( &(x, y)) {
							let mut new = 1;
							if let Some( &n) = pair_map.get( &(x, y)) {
								new = n + 1;}
							pair_map.insert( (x, y), new);}}}}}

		// find pairs above support
		/*let mut result_set : BTreeSet<(u32, u32)> = BTreeSet::new();
		result_set.extend(
			pair_map.iter()
			.filter( | &( &_, &count) | count as f32 > support)
			.map( | ( &x, &_) | x));*/
		let mut result_map : BTreeMap<(u32, u32), u32> = BTreeMap::new();
		result_map.extend( 
			pair_map.iter()
			.filter( | &( &_, &count) | count as f32 > support));

		// clean up and return
		result_map}

	pub fn run( &mut self) -> BTreeMap<(u32, u32), u32> {
		let ( basket_tx, basket_rx) =
			sync_channel::<Vec<u32>>( self.buffer_size);
		let ( term_tx, term_rx) =
			sync_channel::<u32>( 0);
		let support = self.support;

		// run pass one
		// start processing
		if self.debug { println!( "pass one starting...");}
		let handle = thread::spawn( move || {
			ItemSetMiner::pass_one(
				&basket_rx, &term_rx, support)});
		// start reading
		let count = self.reader.run( &basket_tx);
		// send termination signal
		term_tx.send( count).ok();
		// get result
		let ( basket_count, support, freq_items, bucket_map) = handle.join().unwrap();
		if self.debug { println!( "pass one complete");}

		// debug output
		if self.debug { println!( "basket_count: {}", basket_count);}
		if self.debug { println!( "support: {}", support);}
		if self.debug { println!( "freq_items: {:?}", freq_items);}

		// construct candidates
		let mut candidates : Vec<(u32, u32)> = Vec::new();
		for &x in freq_items.iter() {
			for &y in freq_items.iter() {
				if x < y {
					let hash = hash( &( x, y)) as u16;
					if let Some( &hash_check) = bucket_map.get( &hash){
						if hash_check as f32 > support {
							candidates.push( (x, y));}}}}}
		if self.debug { println!( "candidates: {:?}", candidates);}

		// make new channels
		let ( basket_tx, basket_rx) =
			sync_channel::<Vec<u32>>( self.buffer_size);
		let ( term_tx, term_rx) =
			sync_channel::<u32>( 0);

		// run pass two
		// start processing
		if self.debug { println!( "pass two starting...");}
		let handle = thread::spawn( move || {
			ItemSetMiner::pass_two(
				&basket_rx, &term_rx, candidates, support)});
		// start reading
		let count = self.reader.run( &basket_tx);
		// send termination signal
		term_tx.send( count).ok();
		// get result
		let freq_pairs = handle.join().unwrap();
		if self.debug { println!( "pass two complete");}

		// debug output
		if self.debug { println!( "freq_pairs: {:?}", freq_pairs);}

		// return
		freq_pairs}
}