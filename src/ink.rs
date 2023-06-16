#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA {r, g, b, a}
    }

    pub fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        RGBA {r, g, b, a: 255}
    }

	pub fn to_be_bytes(&self) -> Vec<u8> {
		vec![self.r, self.g, self.b, self.a]
	}
}

#[derive(Debug)]
pub struct InkLayer {
    ink_buffer: Vec<RGBA>
}

impl InkLayer {
    pub fn empty() -> Self {
        InkLayer { ink_buffer: vec![] }
    }

    pub fn new(ink_buffer: Vec<RGBA>) -> Self {
        InkLayer { ink_buffer }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for ink in &self.ink_buffer {
            bytes.append(&mut ink.to_be_bytes());
        }
        bytes
    }

    pub fn len(&self) -> usize {
        self.ink_buffer.len()
    }

	pub fn push_ink(&mut self, ink: RGBA) {
		self.ink_buffer.push(ink);
	}

	pub fn get(&self, index: usize) -> Option<&RGBA> {
		self.ink_buffer.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut RGBA> {
		self.ink_buffer.get_mut(index)
	}
}

pub struct Ink;

impl Ink {
	pub const CROSS: RGBA = RGBA { r: 102, g: 120, b: 142, a: 255 };
	pub const TUNNEL: RGBA = RGBA { r: 83, g: 85, b: 114, a: 255 };
	pub const MESH: RGBA = RGBA { r: 100, g: 106, b: 87, a: 255 };
	pub const BUS_0: RGBA = RGBA { r: 122, g: 47, b: 36, a: 255 };
	pub const BUS_1: RGBA = RGBA { r: 62, g: 122, b: 36, a: 255 };
	pub const BUS_2: RGBA = RGBA { r: 36, g: 65, b: 122, a: 255 };
	pub const BUS_3: RGBA = RGBA { r: 37, g: 98, b: 122, a: 255 };
	pub const BUS_4: RGBA = RGBA { r: 122, g: 45, b: 102, a: 255 };
	pub const BUS_5: RGBA = RGBA { r: 122, g: 112, b: 36, a: 255 };
	pub const WRITE: RGBA = RGBA { r: 77, g: 56, b: 62, a: 255 };
	pub const READ: RGBA = RGBA { r: 46, g: 71, b: 93, a: 255 };
	pub const TC_GRAY: RGBA = RGBA { r: 42, g: 53, b: 65, a: 255 };
	pub const TC_WHITE: RGBA = RGBA { r: 159, g: 168, b: 174, a: 255 };
	pub const TC_RED: RGBA = RGBA { r: 161, g: 85, b: 94, a: 255 };
	pub const TC_ORANGE: RGBA = RGBA { r: 161, g: 108, b: 86, a: 255 };
	pub const TC_YELLOW_W: RGBA = RGBA { r: 161, g: 133, b: 86, a: 255 };
	pub const TC_YELLOW_C: RGBA = RGBA { r: 161, g: 152, b: 86, a: 255 };
	pub const TC_LEMON: RGBA = RGBA { r: 153, g: 161, b: 86, a: 255 };
	pub const TC_GREEN_W: RGBA = RGBA { r: 136, g: 161, b: 86, a: 255 };
	pub const TC_GREEN_C: RGBA = RGBA { r: 108, g: 161, b: 86, a: 255 };
	pub const TC_TURQUOISE: RGBA = RGBA { r: 86, g: 161, b: 141, a: 255 };
	pub const TC_BLUE_LIGHT: RGBA = RGBA { r: 86, g: 147, b: 161, a: 255 };
	pub const TC_BLUE: RGBA = RGBA { r: 86, g: 123, b: 161, a: 255 };
	pub const TC_BLUE_DARK: RGBA = RGBA { r: 86, g: 98, b: 161, a: 255 };
	pub const TC_PURPLE: RGBA = RGBA { r: 102, g: 86, b: 161, a: 255 };
	pub const TC_VIOLET: RGBA = RGBA { r: 135, g: 86, b: 161, a: 255 };
	pub const TC_PINK: RGBA = RGBA { r: 161, g: 85, b: 151, a: 255 };
	pub const BUFFER: RGBA = RGBA { r: 146, g: 255, b: 99, a: 255 };
	pub const AND: RGBA = RGBA { r: 255, g: 198, b: 99, a: 255 };
	pub const OR: RGBA = RGBA { r: 99, g: 242, b: 255, a: 255 };
	pub const XOR: RGBA = RGBA { r: 174, g: 116, b: 255, a: 255 };
	pub const NOT: RGBA = RGBA { r: 255, g: 98, b: 138, a: 255 };
	pub const NAND: RGBA = RGBA { r: 255, g: 162, b: 0, a: 255 };
	pub const NOR: RGBA = RGBA { r: 48, g: 217, b: 255, a: 255 };
	pub const XNOR: RGBA = RGBA { r: 166, g: 0, b: 255, a: 255 };
	pub const LATCH_ON: RGBA = RGBA { r: 99, g: 255, b: 159, a: 255 };
	pub const LATCH_OFF: RGBA = RGBA { r: 56, g: 77, b: 71, a: 255 };
	pub const CLOCK: RGBA = RGBA { r: 255, g: 0, b: 65, a: 255 };
	pub const LED: RGBA = RGBA { r: 255, g: 255, b: 255, a: 255 };
	pub const TIMER: RGBA = RGBA { r: 255, g: 103, b: 0, a: 255 };
	pub const RANDOM: RGBA = RGBA { r: 229, g: 255, b: 0, a: 255 };
	pub const BREAKPOINT: RGBA = RGBA { r: 224, g: 0, b: 0, a: 255 };
	pub const WIRELESS_0: RGBA = RGBA { r: 255, g: 0, b: 191, a: 255 };
	pub const WIRELESS_1: RGBA = RGBA { r: 255, g: 0, b: 175, a: 255 };
	pub const WIRELESS_2: RGBA = RGBA { r: 255, g: 0, b: 159, a: 255 };
	pub const WIRELESS_3: RGBA = RGBA { r: 255, g: 0, b: 143, a: 255 };
	pub const ANNOTATION: RGBA = RGBA { r: 58, g: 69, b: 81, a: 255 };
	pub const FILLER: RGBA = RGBA { r: 140, g: 171, b: 161, a: 255 };
	pub const NONE: RGBA = RGBA { r: 0, g: 0, b: 0, a: 0 };
}

pub static TRACES_ORDERED: [RGBA; 16] = [
	RGBA { r: 42, g: 53, b: 65, a: 255 },
	RGBA { r: 159, g: 168, b: 174, a: 255 },
	RGBA { r: 161, g: 85, b: 94, a: 255 },
	RGBA { r: 161, g: 108, b: 86, a: 255 },
	RGBA { r: 161, g: 133, b: 86, a: 255 },
	RGBA { r: 161, g: 152, b: 86, a: 255 },
	RGBA { r: 153, g: 161, b: 86, a: 255 },
	RGBA { r: 136, g: 161, b: 86, a: 255 },
	RGBA { r: 108, g: 161, b: 86, a: 255 },
	RGBA { r: 86, g: 161, b: 141, a: 255 },
	RGBA { r: 86, g: 147, b: 161, a: 255 },
	RGBA { r: 86, g: 123, b: 161, a: 255 },
	RGBA { r: 86, g: 98, b: 161, a: 255 },
	RGBA { r: 102, g: 86, b: 161, a: 255 },
	RGBA { r: 135, g: 86, b: 161, a: 255 },
	RGBA { r: 161, g: 85, b: 151, a: 255 }
];

