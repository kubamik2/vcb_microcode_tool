mod ink;
mod blueprint;
mod microcode;
mod error;

use ink::{Ink, InkLayer, RGBA, TRACES_ORDERED};

use std::io::{Write, Read};
use std::fs::File;
use blueprint::generate_logic_blueprint;

use clap::Parser;
use microcode::{parse_instructions, State, str_to_state_vec};
use serde_json::{from_reader, Value};
use std::collections::HashMap;
use error::ParseError;
use std::path::Path;
use error::Error;

#[derive(Parser)]
struct Cli {
	input: std::path::PathBuf
}

pub struct Config {
	opcodes: u64,
	microcode_map: HashMap<String, i64>,
    opcode_bit_length: u64,
    counter_bit_length: u64,
	counter_starting_number: u64,
	flags_bit_length: u64
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

		let counter_starting_number = value.get("counter_starting_number")
		.ok_or(ParseError::MissingValue("counter_starting_number".to_owned()))?
		.as_u64()
		.ok_or(ParseError::DataType("counter_starting_number".to_owned()))?;

		let flags_bit_length = value.get("flags_bit_length")
		.ok_or(ParseError::MissingValue("flags_bit_length".to_owned()))?
		.as_u64()
		.ok_or(ParseError::DataType("flags_bit_length".to_owned()))?;

		let mut microcode_map: HashMap<String, i64> = HashMap::new();

		for (key, value) in microcode_serde_map {
			let value = value.as_i64().ok_or(ParseError::DataType(key.clone()))?;
			microcode_map.insert(key.clone(), value);
		}

		Ok(Self { opcodes, microcode_map, opcode_bit_length, counter_bit_length, counter_starting_number, flags_bit_length })
	}
}

fn main() -> Result<(), Error> {
	let default_config = include_bytes!("default_config.json");
	if !Path::new("config.json").exists() {
		File::create("config.json").unwrap().write_all(default_config).unwrap();
	}
	let args = Cli::parse();

	let mut input_file = File::open(args.input)?;
	let mut input = String::new();
	input_file.read_to_string(&mut input)?;

	println!("{}", generate_blueprint(&input)?);
	Ok(())
}

fn append_state_vec_to_ink_layer(state_vec: &Vec<State>, ink_layer: &mut InkLayer, gate_ink: RGBA) {
	for state in state_vec {
		if gate_ink == Ink::AND {
			match state {
				State::True => {
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
	
					ink_layer.ink_buffer.push(Ink::READ);
	
					ink_layer.ink_buffer.push(gate_ink);
					
				},
				State::False => {
					ink_layer.ink_buffer.push(Ink::READ);
					ink_layer.ink_buffer.push(gate_ink);
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
				},
				State::Any => {
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
				}
			}
		} else {
			match state {
				State::False => {
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
	
					ink_layer.ink_buffer.push(Ink::READ);
	
					ink_layer.ink_buffer.push(gate_ink);
					
				},
				State::True => {
					ink_layer.ink_buffer.push(Ink::READ);
					ink_layer.ink_buffer.push(gate_ink);
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
				},
				State::Any => {
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
					ink_layer.ink_buffer.push(Ink::TC_GRAY);
					ink_layer.ink_buffer.push(gate_ink);
				}
			}
		}
		
	}
}

fn generate_blueprint(input: &String) -> Result<String, Error> {
	let config_file = File::open("config.json")?;
	let config_serde: Value = from_reader(config_file)?;
	let config = Config::try_from(config_serde)?;

	let instructions = parse_instructions(&input, &config)?;

	let mut ink_buffer: InkLayer = InkLayer::empty();
	let mut height: u32 = 0;
	let max_index = config.microcode_map.values().max().ok_or(ParseError::MissingValue("microcodes".to_owned()))?.clone() as u64;
	let width: u32 = (config.opcodes * config.opcode_bit_length * 4 + config.counter_bit_length * 4 + (max_index + 1) * 2 + config.flags_bit_length * 4) as u32;
	let mut gate_ink = Ink::AND;

	for instruction in instructions {
		let mut counter = config.counter_starting_number;
		for layer in &instruction.microcodes {
			for _ in 0..width/2 {
				ink_buffer.ink_buffer.push(Ink::CROSS);
				ink_buffer.ink_buffer.push(gate_ink);
			}
			height += 1;

			for opcode in &instruction.opcodes {
				append_state_vec_to_ink_layer(opcode, &mut ink_buffer, gate_ink);
			}
			let counter_string = format!("{:0>width$b}", counter, width = config.counter_bit_length.clone() as usize);
			let counter_state_vec = str_to_state_vec(&counter_string)?;

			append_state_vec_to_ink_layer(&counter_state_vec, &mut ink_buffer, gate_ink);
			for i in (1..=config.flags_bit_length * 2).rev() {
				if layer.contains(&-(i as i64)) {
					ink_buffer.ink_buffer.push(Ink::READ);
				} else {
					ink_buffer.ink_buffer.push(Ink::TC_GRAY);
				}
				ink_buffer.ink_buffer.push(gate_ink);
			}
			height += 1;

			for i in 0..=max_index {
				if layer.contains(&(i as i64)) {
					ink_buffer.ink_buffer.push(Ink::WRITE);
				} else {
					ink_buffer.ink_buffer.push(TRACES_ORDERED[i as usize % 16]);
				}
				ink_buffer.ink_buffer.push(gate_ink);
			}
			if gate_ink == Ink::AND {
				gate_ink = Ink::NOR;
			} else {
				gate_ink = Ink::AND
			}
			counter += 1;
		}
	}

	generate_logic_blueprint(&ink_buffer, width, height)
}