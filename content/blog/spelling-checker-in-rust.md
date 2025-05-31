+++
title = "Spelling Checker in Rust"
date = "2016-05-06"

[taxonomies]
tags=["Programming"]
+++

Peter Norvig (yes, that Peter Norvig) wrote a brief blog post about building a spellcheck application. It's a beautifully simple approach which demonstrates the unreasonable effectiveness of simple frequency and edit distance tricks. His original blog post can be read here: <http://norvig.com/spell-correct.html>I decided to write a version of my own in Rust to learn the language.The full GitHub project with Cargo and wordlist is here: <https://github.com/JosephCatrambone/RustSpellcheck>And the Rust code of interest:

```

use std::io;
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

static WORD_FILE: &'static str = "words.txt";
static QUIT_COMMAND: &'static str = "quit";

fn edits(word : &str) -> Vec<string>{
	let alphabet = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
	let mut edits = Vec::<string>::new();
	// Find corruptions of word
	for i in 0..word.len() {
		let (a, b) = word.split_at(i);
		// Deletions
		if b.len() > 0 {
			edits.push(a.to_string() + &b[1..]);
		}
		// Transpositions
		if b.len() > 1 {
			let mut transposition = a.to_string();
			transposition.push(b.chars().nth(1).expect("Panic while building character transposition.  Unable to decode character."));
			transposition.push(b.chars().nth(0).expect("Panic while building character transposition.  Unable to decode character."));
			transposition.push_str(&b[2..]);
			edits.push(transposition);
		}
		// Replacements
		if b.len() > 0 {
			for character in &alphabet {
				edits.push(a.to_string() + &character + &b[1..]);
			}
		}
		// Insertions
		for character in &alphabet {
			edits.push(a.to_string() + &character + b);
		}
	}
	// &String can automatically coerce to &str, but str -> String
	edits
}

fn update_frequency_count(model : &mut HashMap<string,u64>, words : String) -> () {
	let word_iterator = words.split_whitespace();
	// TODO: Find a more generic iterator.
	for word in word_iterator {
		let lower_word = word.to_lowercase();
		let count = model.entry(lower_word).or_insert(0);
		*count += 1;
	}
}

fn correct(model : &HashMap<string,u64>, word : String) -> String {
	// If the word is spelled right, return it.
	if model.contains_key(&word) {
		return word;
	}

	// Allocate some placeholders for our frequency and best match.
	let mut best_match = String::new();
	let mut frequency : u64 = 0;

	// First degree corruption
	// Get the corruptions of each
	let corruptions = edits(&word);
	for corruption in &corruptions { // &word so it casts to &str.
		match model.get(&corruption.to_string()) {
			Some(f2) => {
				if *f2 > frequency {
					best_match = corruption.to_string();
					frequency = *f2;
				}
			},
			None => {}
		}
	}
	if frequency > 0 {
		return best_match;
	}

	// Second degree corruption
	// Frequency is still zero if we're here.
	for corruption in &corruptions {
		let double_corruptions = edits(&corruption);
		for c2 in &double_corruptions {
			match model.get(&c2.to_string()) {
				Some(freq) => {
					if *freq > frequency {
						best_match = c2.to_string();
						frequency = *freq;
					}
				},
				None => {}
			}
		}
	}
	if frequency > 0 {
		return best_match;
	}

	// No matches at all.
	println!("No match.");
	word
}

fn main() {
	// Read words.
	let mut fin = File::open(WORD_FILE).unwrap();
	let mut lines = String::new();
	fin.read_to_string(&mut lines).unwrap(); // Just bury read errors.

	// Gather words into hash table.
	let mut model = HashMap::<string,u64>::new();
	update_frequency_count(&mut model, lines);

	loop {
		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Problem reading from stdin.");
		user_input = user_input.trim().to_lowercase().to_string();
		if user_input.trim() == QUIT_COMMAND {
			break;
		}
		let correction = correct(&model, user_input.to_string()).to_string();
		println!("{}", correction);
	}
}
</string,u64></string,u64></string,u64></string></string></code>
```

I should give the caveat that this is probably not idiomatic Rust. It's probably not even particularly good Rust. Such is the way of the web, though. I hope it proves useful for someone.
