use std::ops::RangeInclusive;

use tokio::net::TcpStream;
use llhttp_rs::{Parser, Callbacks};

// static ASCII_ZEROS: [u8; 256] = [
//   // NUL  SOH  STX  ETX  EOT  ENQ  ACK  BEL  BS   TAB
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // LF   VT   FF   CR   SO   SI   DLE  DC1  DC2  DC3
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // DC4  NAK  SYN  ETB  CAN  EM   SUB  ESC  FS   GS
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // RS   US   SP   !  "  #  $  %  &  '
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // (  )  *  +  ,  -  .  /  0  1
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // 2  3  4  5  6  7  8  9  :  ;
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // <  =  >  ?  @  A  B  C  D  E
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // F  G  H  I  J  K  L  M  N  O
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // P  Q  R  S  T  U  V  W  X  Y
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // Z  [  \  ]  ^  _  `  a  b  c
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // d  e  f  g  h  i  j  k  l  m
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // n  o  p  q  r  s  t  u  v  w
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // x  y  z  {  |  }  ~  DEL
//   0,  0,   0,   0,   0,   0,   0,   0,
//   // Extended ASCII (128-255)
//   // €  �  ‚  ƒ  „  …  †  ‡  ˆ  ‰
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // Š  ‹  Œ  �  Ž  �  �  ‘  ’  “
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ”  •  –  —  ˜  ™  š  ›  œ  �
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ž  Ÿ  ¡  ¢  £  ¤  ¥  ¦  §  ¨
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ©  ª  «  ¬  ®  ¯  °  ±  ²  ³
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ´  µ  ¶  ·  ¸  ¹  º  »  ¼  ½
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ¾  ¿  À  Á  Â  Ã  Ä  Å  Æ  Ç
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // È  É  Ê  Ë  Ì  Í  Î  Ï  Ð  Ñ
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // Ò  Ó  Ô  Õ  Ö  ×  Ø  Ù  Ú  Û
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // Ü  Ý  Þ  ß  à  á  â  ã  ä  å
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // æ  ç  è  é  ê  ë  ì  í  î  ï
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ð  ñ  ò  ó  ô  õ  ö  ÷  ø  ù
//   0,  0,   0,   0,   0,   0,   0,   0,   0,   0,
//   // ú  û  ü  ý  þ  ÿ
//   0,  0,   0,   0,   0,   0
// ];

// pub static X: [bool; 32] = [
//   //	Null character
// 	false,
// 	// Start of Heading
// 	false,
// 	// Start of Text
// 	false,
// 	// End of Text
// 	false,
// 	// End of Transmission
// 	false,
// 	// Enquiry
// 	false,
// 	// Acknowledge
// 	false,
// 	// Bell, Alert
// 	false,
// 	// Backspace
// 	false,
// 	// Horizontal Tab
// 	false,
// 	// Line Feed
// 	false,
// 	// Vertical Tabulation
// 	false,
// 	// Form Feed
// 	false,
// 	// Carriage Return
// 	false,
// 	// Shift Out
// 	false,
// 	// Shift In
// 	false,
// 	// Data Link Escape
// 	false,
// 	// Device Control One (XON)
// 	false,
// 	// Device Control Two
// 	false,
// 	// Device Control Three (XOFF)
// 	false,
// 	// Device Control Four
// 	false,
// 	// Negative Acknowledge
// 	false,
// 	// Synchronous Idle
// 	false,
// 	// End of Transmission Block
// 	false,
// 	// Cancel
// 	false,
// 	// End of medium
// 	false,
// 	// Substitute
// 	false,
// 	// Escape
// 	false,
// 	// File Separator
// 	false,
// 	// Group Separator
// 	false,
// 	// Record Separator
// 	false,
// 	// Unit Separator
// 	false,
// ];

// ALPHA: https://tools.ietf.org/html/rfc5234#appendix-B.1
static ALPHA: LookupTable = LookupTable::new()
  .with_byte_range(b'a'..=b'z')
	.with_byte_range(b'A'..=b'Z');

static DIGIT: LookupTable = LookupTable::new()
	.with_byte_range(b'0'..=b'9');

static ALPHANUM: LookupTable = LookupTable::new()
  .with_table(&ALPHA)
	.with_table(&DIGIT);

static MARK: LookupTable = LookupTable::new()
	.with_bytes(b"-_.!~*'()");

static USERINFO_CHARACTER: LookupTable = LookupTable::new()
	.with_table(&ALPHANUM)
	.with_table(&MARK)
	.with_bytes(b"%;:&=+$,");

static URL_CHARACTER: LookupTable = LookupTable::new()
	.with_table(&ALPHANUM)
	.with_bytes(b"!\"$%&'()*+,-./:;<=>@[\\]^_`{|}~");

pub fn is_alpha(byte: u8) -> bool {
  byte >= b'a' && byte <= b'z'
	||
  byte >= b'A' && byte <= b'Z'
}

// DIGIT: https://tools.ietf.org/html/rfc5234#appendix-B.1
pub fn is_digit(byte: u8) -> bool {
	byte >= b'0' && byte <= b'9'
}

pub fn is_alphanum(byte: u8) -> bool {
	is_alpha(byte) || is_digit(byte)
}

pub fn is_mark(byte: u8) -> bool {
	matches!(byte, 
		b'-'
		| b'_'
		| b'.'
		| b'!'
		| b'~'
		| b'*'
		| b'\''
		| b'('
		| b')' 
	)
}

pub fn is_unserinfo_byte(byte: u8) -> bool {
	is_alphanum(byte)
	||
	is_mark(byte)
	||
	matches!(byte, b'%' | b';' | b':' | b'&' | b'=' | b'+' | b'$' | b',')
}

pub fn is_url_character(byte: u8) -> bool {
	is_alphanum(byte)
	||
  matches!(byte,  
		b'!' | b'"' | b'$' | b'%' | b'&' | b'\'' | 
		b'(' | b')' | b'*' | b'+' | b',' | b'-' | 
		b'.' | b'/' | b':' | b';' | b'<' | b'=' | 
		b'>' | b'@' | b'[' | b'\\' | b']' | b'^' | 
		b'_' | b'`' | b'{' | b'|' | b'}' | b'~'
	)
}

pub fn is_hex(byte: u8) -> bool {
  is_digit(byte)
	||
	matches!(byte, b'a' | b'b' | b'c' | b'd' | b'e' | b'f' | b'A' | b'B' | b'C' | b'D' | b'E' | b'F')
}

pub fn is_token(byte: u8) -> bool {
	// is_
	// ...ALPHANUM
	//   '!', '#', '$', '%', '&', '\'',
  // '*', '+', '-', '.',
  // '^', '_', '`',
  // '|', '~',
	true
}

struct LookupTable {
	table: [bool; 256],
}

impl LookupTable {
	pub const fn new() -> Self {
		Self {
			table: [false; 256],
		}
	}

	pub const fn with_byte(mut self, byte: u8) -> Self {
		self.table[byte as usize] = true;
		self
	}

	pub const fn with_bytes(mut self, bytes: &[u8]) -> Self {
		let mut index = 0;

		while index < bytes.len() {
			if index >= bytes.len() {
				break;
			}

			self.table[bytes[index] as usize] = true;
			index += 1;
		}

		self
	}

	pub const fn with_byte_range(mut self, range: RangeInclusive<u8>) -> Self {
    let mut index = *range.start();
    
    while index <= *range.end() {
			self.table[index as usize] = true;
			index += 1;
    }

		self
	}

	pub const fn with_table(mut self, table: &LookupTable) -> Self {
    let mut index = 0;
    
    while index < table.table.len() {
			if table.table[index as usize] == true {
				self.table[index as usize] = true;
			}
			index += 1;
    }

		self
	}

	pub const fn from_bytes(bytes: &[u8]) -> Self {
		let x = b'0'..=b'9';

    let mut table = [true; 256];
		let mut index = 0;

		while index < bytes.len() {
			if index >= bytes.len() {
				break;
			}

			table[bytes[index] as usize] = true;
			index += 1;
		}

		Self {
			table
		}
	}
}

fn tt() {
	let x: Vec<u8> =( b'0'..=b'9').into_iter().collect();
}