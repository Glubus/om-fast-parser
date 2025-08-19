use crate::structs::{HitObject, HitObjectType};

// Version byte-level (reprend l'ancien nom)
#[inline(always)]
pub fn parse_int_fast(bytes: &[u8]) -> i32 {
	let mut i = 0;
	let mut result = 0;
	let mut negative = false;
	
	// Skip whitespace
	while i < bytes.len() && bytes[i] == b' ' {
		i += 1;
	}
	
	// Check sign
	if i < bytes.len() && bytes[i] == b'-' {
		negative = true;
		i += 1;
	}
	
	// Loop unrolling par 4
	let len = bytes.len();
	while i + 3 < len {
		let b0 = bytes[i];
		let b1 = bytes[i + 1];
		let b2 = bytes[i + 2];
		let b3 = bytes[i + 3];
		
		if b0 >= b'0' && b0 <= b'9' && b1 >= b'0' && b1 <= b'9' && 
			b2 >= b'0' && b2 <= b'9' && b3 >= b'0' && b3 <= b'9' {
			result = result * 10000 + 
					(b0 - b'0') as i32 * 1000 + 
					(b1 - b'0') as i32 * 100 + 
					(b2 - b'0') as i32 * 10 + 
					(b3 - b'0') as i32;
			i += 4;
		} else {
			break;
		}
	}
	
	// Reste des digits
	while i < len {
		let byte = bytes[i];
		if byte >= b'0' && byte <= b'9' {
			result = result * 10 + (byte - b'0') as i32;
		} else {
			break;
		}
		i += 1;
	}
	
	if negative { -result } else { result }
}

pub fn parse_hit_object_line(line: &[u8], mode: u8) -> HitObject {
	let mut start = 0usize;
	let mut end = 0usize;
	
	// x
	while end < line.len() && line[end] != b',' { end += 1; }
	let x = parse_int_fast(&line[start..end]);
	
	// y
	start = end + 1; end = start;
	while end < line.len() && line[end] != b',' { end += 1; }
	let y = parse_int_fast(&line[start..end]);
	
	// time
	start = end + 1; end = start;
	while end < line.len() && line[end] != b',' { end += 1; }
	let time = parse_int_fast(&line[start..end]);
	
	// type_flags
	start = end + 1; end = start;
	while end < line.len() && line[end] != b',' { end += 1; }
	let type_flags = parse_int_fast(&line[start..end]) as u8;
	
	// object type
	let object_type = if mode == 3 && (type_flags & 128) != 0 { HitObjectType::Hold } else { HitObjectType::Circle };
	
	// duration
	let end_time = if matches!(object_type, HitObjectType::Hold) {
		// skip hitSound
		start = end + 1; end = start;
		while end < line.len() && line[end] != b',' { end += 1; }
		// duration until ':' or ','
		start = end + 1; end = start;
		while end < line.len() && line[end] != b':' && line[end] != b',' { end += 1; }
		Some(parse_int_fast(&line[start..end]))
	} else {
		None
	};
	
	HitObject { x, y, time, object_type, end_time }
}
