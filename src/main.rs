mod ink;
mod blueprint;
mod microcode;
use ink::{Ink, InkLayer};

use std::io::{Write, Read};
use std::fs::File;
use std::str::Chars;
use blueprint::generate_logic_blueprint;

use clap::Parser;
use microcode::MICROCODE_MAP;
use regex::Regex;
use anyhow::anyhow;

type Error = Box<dyn std::error::Error>;

#[derive(Parser)]
struct Cli {
	input: std::path::PathBuf,
	output: std::path::PathBuf
}

fn main() -> Result<(), Error> {
	let args = Cli::parse();

	let mut input_file = File::open(args.input)?;
	let mut output_file = File::create(args.output)?;
	let mut input = String::new();
	input_file.read_to_string(&mut input)?;



	dbg!(microcode_to_blueprint(input));
	Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
	True,
	False,
	Any
}

impl Default for State {
	fn default() -> Self {
		Self::False
	}
}

type Instruction = ([State; 6], [State; 6], Vec<Vec<u8>>);

fn microcode_to_blueprint(input: String) -> Result<(), Error> {
	let exp = Regex::new("(?P<opcode>[01xX]{6}) *(?P<funct>[01xX]{6})[\r\n ]\n?(?P<microcode>([A-Z_]*[\r\n ]?[\n]?)*)").unwrap();
	let mut instructions: Vec<Instruction> = vec![];
	for captures in exp.captures_iter(&input) {
		let opcode = captures.name("opcode").ok_or(anyhow!("Opcode is missing"))?.as_str();
		let funct = captures.name("funct").ok_or(anyhow!("Funct is missing"))?.as_str();
		let unformatted_microcode = captures.name("microcode").ok_or(anyhow!("Microcode is missing"))?.as_str();
		let exp2 = Regex::new("([A-Z_]* ?)*").unwrap();
		
		let mut microcode: Vec<Vec<u8>> = vec![];
		for captures2 in exp2.captures_iter(unformatted_microcode) {
			let micro_instruction = captures2.get(1).ok_or(anyhow!("1st index is missing"))?.as_str();
			match MICROCODE_MAP.get(micro_instruction) {
				Some(microcode_index) => {
					microcode.push(*microcode_index);
				},
				None => {return Err(anyhow!("Unkown micro instruction").into());}
			}
		};
		let instruction: Instruction = (parse_head(opcode), parse_head(funct), microcode);
	}
	Ok(())
}

fn parse_head(input: &str) -> [State; 6] {
	let mut output: [State; 6] = Default::default();
	for (i, c) in input.chars().enumerate() {
		match c {
			'1' => output[i] = State::True,
			'x'|'X' => output[i] = State::Any,
			_ => ()
		}
	}
	output
}