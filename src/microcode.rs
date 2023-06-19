use nom::{IResult, character::complete::{ multispace1, not_line_ending, one_of, space1, digit1}, multi::{many1, separated_list1}, bytes::complete::tag, sequence::{preceded, delimited, terminated, pair}, combinator::map, branch::alt};
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

#[derive(Debug, Clone)]
pub struct Operation {
    pub counter: u32,
    pub micro_operations: Vec<i64>
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcodes: Vec<Vec<State>>,
    pub operations: Vec<Operation>
}

impl Instruction {
    pub fn new() -> Self {
        Self { opcodes: vec![], operations: vec![] }
    }
}

fn parse_comment(input: &str) -> IResult<&str, &str> {
    preceded(tag("//"), not_line_ending)(input)
}

fn parse_multispace(input: &str) -> IResult<&str, String> {
    map(
        many1(alt((map(parse_comment, |_| ""), multispace1))),
        |f| f.join("")
    )(input)
}

fn parse_word(input: &str) -> IResult<&str, String> {
    map(
        many1(one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-!")),
        |f| f.iter().collect::<String>()
    )(input)
}

fn parse_space(input: &str) -> IResult<&str, String> {
    map(
        many1(alt((map(parse_comment, |_| ""), space1))),
        |f| f.join("")
    )(input)
}

fn parse_counter(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn parse_opcode(input: &str) -> IResult<&str, String> {
    map(
        many1(one_of("10#")),
        |f| f.iter().collect::<String>()
    )(input)
}

fn parse_opcodes<'a>(input: &'a str, config: &'a Config) -> Result<(&'a str, Vec<Vec<State>>), ParseError> {
    let result = delimited(tag("["), separated_list1(parse_space, parse_opcode), tag("]"))(input).map_err(|_| ParseError::OpcodeFormatting)?;
    let opcodes = result.1.iter()
    .map(|f| 
        str_to_state_vec(f)
        .map(|op| 
            if op.len() as u64 != config.opcode_bit_length {
                Err(ParseError::OpcodeLength)
            } else {
                Ok(op)
            })
    ).collect::<Result<Result<Vec<Vec<State>>, ParseError>, ParseError>>()??;
    Ok((result.0, opcodes))
}

fn parse_operation_line<'a>(input: &'a str, config: &'a Config) -> Result<(&'a str, Operation), ParseError> {
    let result = pair(terminated(parse_counter, parse_space), separated_list1(parse_space, parse_word))(input).map_err(|_| ParseError::InstructionFormatting)?;
    let counter = result.1.0.parse::<u32>().map_err(|_| ParseError::CounterFormatting)?;
    let micro_operations = result.1.1.iter()
        .map(|f| 
            config.microcode_map.get(f)
            .ok_or(ParseError::MissingInstruction(f.clone())).map(|op| op.clone()))
        .collect::<Result<Vec<i64>, ParseError>>()?;
    Ok((result.0, Operation { counter, micro_operations }))
}

pub fn parse_instruction(input: &str, config: &Config) -> Result<(String, Instruction), ParseError> {
    let mut input = input.to_owned();

    let mut instruction = Instruction::new();
    let (rest, opcodes) = parse_opcodes(&input, config)?;
    instruction.opcodes = opcodes;
    input = rest.to_owned();

    loop {
        let (rest, _) = parse_multispace(&input).map_err(|_| ParseError::InstructionFormatting)?;
        if rest.starts_with("[") || rest.is_empty() {break;}
        input = rest.to_owned();

        let (rest, operation) = parse_operation_line(&input, config)?;
        input = rest.to_owned();
        if operation.counter > 2u32.pow(config.counter_bit_length as u32) - 1 {return Err(ParseError::CounterOverflow);}
        instruction.operations.push(operation);
        if input.is_empty() {break;}
    }

    Ok((input, instruction))
}

pub fn parse_instructions(input: &str, config: &Config) -> Result<Vec<Instruction>, ParseError> {
    let mut input = input.to_owned();
    let mut instructions = vec![];
    if let Ok((rest, _)) = parse_multispace(&input) {
        input = rest.to_owned();
    }

    while input.len() > 0 {
        let (output, instruction) = parse_instruction(&input, &config)?;
        input = output;
        instructions.push(instruction);
        if let Ok((rest, _)) = parse_multispace(&input) {
            input = rest.to_owned();
        }
    }
    Ok(instructions)
}