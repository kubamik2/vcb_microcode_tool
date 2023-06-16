use nom::{IResult, character::complete::{ multispace1, not_line_ending, one_of, space1}, multi::many1};
use crate::{error::ParseError, Config};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	True,
	False,
	Any
}

pub fn str_to_state_vec(input: &str) -> Result<Vec<State>, ParseError> {
    let mut result = vec![];
    for ch in input.chars() {
        match ch {
            '0' => { result.push(State::False) },
            '1' => { result.push(State::True) },
            '#' => { result.push(State::Any) },
            _ => {return Err(ParseError::Formatting);}
        }
    }
    Ok(result)
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
        let (_, opcode_str) = parse_opcode(&input).map_err(|_| ParseError::OpcodeFormatting)?;

        if opcode_str.len() as u64 != config.opcode_bit_length {return Err(ParseError::OpcodeLength);}

        let opcode = str_to_state_vec(&opcode_str).map_err(|_| ParseError::OpcodeFormatting)?;

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
        if instruction.microcodes.len() as u64 > (2 as u64).pow(config.counter_bit_length as u32) - 1 + config.counter_starting_number - 1 {
            return Err(ParseError::InstructionLength);
        }
    }
    Ok((input, instruction))
}

pub fn parse_instructions(input: &str, config: &Config) -> Result<Vec<Instruction>, ParseError> {
    let mut input = input.to_owned();
    let mut instructions: Vec<Instruction> = vec![];

    loop {
        let (output, instruction) = parse_instruction(&input, config)?;
        input = output;
        instructions.push(instruction);
        if input.is_empty() {break;}
    }

    Ok(instructions)
}