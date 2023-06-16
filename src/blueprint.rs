use crate::ink::InkLayer;
use zstd::Encoder;
use base64::engine::{general_purpose::STANDARD, Engine};
use sha1_smol::Sha1;
use std::io::Write;
use crate::error::Error;

pub fn generate_logic_blueprint(ink_buffer: &InkLayer, width: u32, height: u32) -> Result<String, Error> {
	let mut blueprint = String::new();
	blueprint.push_str("VCB+AAAA");
	let mut blueprint_chunk: Vec<u8> = vec![];

	blueprint_chunk.append(&mut width.to_be_bytes().to_vec());
	blueprint_chunk.append(&mut height.to_be_bytes().to_vec());

	let mut compressed_buffer: Vec<u8> = vec![];
	let mut encoder = Encoder::new(&mut compressed_buffer, 22)?;
	encoder.write_all(&ink_buffer.to_be_bytes())?;
	encoder.finish()?;

	blueprint_chunk.append(&mut (compressed_buffer.len() as u32 + 12).to_be_bytes().to_vec());
	blueprint_chunk.append(&mut vec![0,0,0,0]);
	blueprint_chunk.append(&mut (ink_buffer.to_be_bytes().len() as u32).to_be_bytes().to_vec());
	blueprint_chunk.append(&mut compressed_buffer);
	
	let base64_blueprint_chunk = STANDARD.encode(&blueprint_chunk);

	let mut hasher = Sha1::new();
	hasher.update(base64_blueprint_chunk.as_bytes());
	let hash = &hasher.digest().bytes()[0..6];

	blueprint.push_str(&STANDARD.encode(&hash));
	blueprint.push_str(&base64_blueprint_chunk);

	Ok(blueprint)
}