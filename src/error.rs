use std::{ error::Error as StdError, fmt::Display };

pub type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum ParseError {
    OpcodeFormatting,
    OpcodeLength,
    MissingInstruction(String),
    InstructionFormatting,
    DataType(String),
    MissingValue(String)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::OpcodeFormatting => {write!(f, "Invalid opcode formatting")},
            ParseError::OpcodeLength => {write!(f, "Invalid opcode length")},
            ParseError::MissingInstruction(instruction) => {write!(f, "Instruction '{}' doesn't exist", instruction)},
            ParseError::DataType(key) => {write!(f, "Invalid data type for '{}'", key)},
            ParseError::MissingValue(value) => {write!(f, "Value '{}' doesn't exist", value)},
            ParseError::InstructionFormatting => {write!(f, "Invalid instruction formatting")}
        }
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::OpcodeFormatting => "Invalid opcode formatting",
            ParseError::OpcodeLength => "Invalid opcode length",
            ParseError::MissingInstruction(_) => "Instruction doesn't exist",
            ParseError::DataType(_) => "Invalid data type",
            ParseError::MissingValue(_) => "Missing value",
            ParseError::InstructionFormatting => "Invalid instruction formatting"
        }
    }
}