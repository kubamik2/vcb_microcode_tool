use nom::{IResult, character::complete::{ multispace1, not_line_ending, one_of, space1}, multi::many1};
use crate::{error::ParseError, Config};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	True,
	False,
	Any
}

impl Default for State {
	fn default() -> Self {
		Self::False
	}
}

#[derive(Debug)]
pub struct Instruction {
    pub opcodes: Vec<Vec<State>>,
    pub microcodes: Vec<Vec<u64>>
}

impl Instruction {
    pub fn new() -> Self {
        Self { opcodes: vec![], microcodes: vec![] }
    }
}

fn parse_opcode(input: &str) -> IResult<&str, String> {
    many1(one_of::<&str, _, nom::error::Error<&str>>("10#"))(&input)
    .map(|op| (op.0, op.1.iter().collect::<String>()))
}

fn parse_multispace(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}

fn parse_line(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

fn parse_word(input: &str) -> IResult<&str, String> {
    many1(one_of::<&str, _, nom::error::Error<&str>>("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-"))(&input)
    .map(|op| (op.0, op.1.iter().collect::<String>()))
}

fn parse_space(input: &str) -> IResult<&str, &str> {
    space1(input)
}

fn parse_micro_operations(line: String, config: &Config) -> Result<Vec<u64>, ParseError> {
    let mut line = line;
    let mut microcode_layer: Vec<u64> = vec![];
    
    while let Ok((rest, chunk)) = parse_word(&line) {
        let index = config.microcode_map.get(&chunk).ok_or(ParseError::MissingInstruction(chunk))?.clone();
        microcode_layer.push(index);

        line = rest.to_owned();
        match parse_space(&line) {
            Ok((rest, _)) => {
                line = rest.to_owned();
            },
            Err(_) => {
                break;
            }
        }
    }
    Ok(microcode_layer)
}

pub fn parse_instruction(input: &str, config: &Config) -> Result<(String, Instruction), ParseError> {
    let mut input = input.to_owned();
    let mut instruction = Instruction::new();
    for _ in 1..=config.opcodes {
        let mut opcode: Vec<State> = vec![];
        let (_, opcode_str) = parse_opcode(&input).map_err(|_| ParseError::OpcodeFormatting)?;

        if opcode_str.len() as u64 != config.opcode_bit_length {return Err(ParseError::OpcodeLength);}
        for ch in opcode_str.chars() {
            match ch {
                '0' => {opcode.push(State::False)},
                '1' => {opcode.push(State::True)},
                '#' => {opcode.push(State::Any)}
                _ => {return Err(ParseError::OpcodeFormatting);}
            }
        }
        instruction.opcodes.push(opcode);

        input.drain(0..config.opcode_bit_length as usize);

        let (rest, _) = parse_multispace(&input).map_err(|_| ParseError::OpcodeFormatting)?;
        input = rest.to_owned();
    }

    
    loop {
        let (rest, line) = parse_line(&input).map_err(|_| ParseError::InstructionFormatting)?;
        let line = line.to_owned();
        
        let microcode_layer = parse_micro_operations(line, &config)?;
        if microcode_layer.len() == 0 {break;}

        input = rest.to_owned();
        if let Ok((rest, _)) = parse_multispace(&input) {
            input = rest.to_owned();
        }
        instruction.microcodes.push(microcode_layer);
    }
    Ok((input, instruction))
}

pub fn parse_instructions(input: &str, config: &Config) -> Result<Vec<Instruction>, ParseError> {
    let mut input = input.to_owned();
    let mut instructions: Vec<Instruction> = vec![];
    while let Ok((output, instruction)) = parse_instruction(&input, config) {
        input = output;

        instructions.push(instruction);
    }
    Ok(instructions)
}