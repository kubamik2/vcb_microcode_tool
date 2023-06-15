mod ink;
mod blueprint;
mod microcode;
mod error;

use ink::{Ink, InkLayer};

use std::io::{Write, Read};
use std::fs::File;
use blueprint::generate_logic_blueprint;

use clap::Parser;
use microcode::parse_instructions;
use serde_json::{from_reader, Value};
use std::collections::HashMap;
use error::ParseError;
use std::path::Path;
use error::Error;

#[derive(Parser)]
struct Cli {
	input: std::path::PathBuf,
	output: std::path::PathBuf
}

pub struct Config {
	opcodes: u64,
	microcode_map: HashMap<String, u64>,
    opcode_bit_length: u64,
    counter_bit_length: u64
}

impl TryFrom<Value> for Config {
	type Error = Error;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		let opcodes = value.get("opcodes")
			.ok_or(ParseError::MissingValue("opcodes".to_owned()))?
			.as_u64()
			.ok_or(ParseError::DataType("opcodes".to_owned()))?;

		let opcode_bit_length = value.get("opcode_bit_length")
			.ok_or(ParseError::MissingValue("opcode_bit_length".to_owned()))?
			.as_u64()
			.ok_or(ParseError::DataType("opcode_bit_length".to_owned()))?;

		let counter_bit_length = value.get("counter_bit_length")
			.ok_or(ParseError::MissingValue("counter_bit_length".to_owned()))?
			.as_u64()
			.ok_or(ParseError::DataType("counter_bit_length".to_owned()))?;

		let microcode_serde_map = value.get("microcodes")
			.ok_or(ParseError::MissingValue("microcodes".to_owned()))?
			.as_object()
			.ok_or(ParseError::MissingValue("microcodes".to_owned()))?;
		let mut microcode_map: HashMap<String, u64> = HashMap::new();

		for (key, value) in microcode_serde_map {
			let value = value.as_u64().ok_or(ParseError::DataType(key.clone()))?;
			microcode_map.insert(key.clone(), value);
		}

		Ok(Self { opcodes, microcode_map, opcode_bit_length, counter_bit_length })
	}
}

fn main() -> Result<(), Error> {
	let default_config = include_bytes!("default_config.json");
	if !Path::new("config.json").exists() {
		File::create("config.json").unwrap().write_all(default_config).unwrap();
	}
	let args = Cli::parse();

	let mut input_file = File::open(args.input)?;
	let mut output_file = File::create(args.output)?;
	let mut input = String::new();
	input_file.read_to_string(&mut input)?;

	let config_file = File::open("config.json")?;
	let config_serde: Value = from_reader(config_file)?;
	let config = Config::try_from(config_serde)?;

	dbg!(parse_instructions(&input, &config));
	Ok(())
}