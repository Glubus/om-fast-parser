use crate::structs::{HitObject, HitObjectType};

#[inline(always)]
pub fn parse_int_fast(s: &str) -> i32 {
    let bytes = s.as_bytes();
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
    
    // Parse digits
    while i < bytes.len() {
        let byte = bytes[i];
        if byte >= b'0' && byte <= b'9' {
            result = result * 10 + (byte - b'0') as i32;
        } else {
            break;
        }
        i += 1;
    }
    
    if negative {
        -result
    } else {
        result
    }
}

#[inline(always)]
pub fn parse_hit_object_line(line: &str, mode: u8) -> Result<HitObject, Box<dyn std::error::Error>> {
    // Parsing ultra-rapide avec des indices
    let mut start = 0;
    let mut end = 0;
    
    // Parse x
    while end < line.len() && line.as_bytes()[end] != b',' {
        end += 1;
    }
    let x = parse_int_fast(&line[start..end]);
    
    // Parse y
    start = end + 1;
    end = start;
    while end < line.len() && line.as_bytes()[end] != b',' {
        end += 1;
    }
    let y = parse_int_fast(&line[start..end]);
    
    // Parse time
    start = end + 1;
    end = start;
    while end < line.len() && line.as_bytes()[end] != b',' {
        end += 1;
    }
    let time = parse_int_fast(&line[start..end]);
    
    // Parse type_flags
    start = end + 1;
    end = start;
    while end < line.len() && line.as_bytes()[end] != b',' {
        end += 1;
    }
    let type_flags = parse_int_fast(&line[start..end]) as u8;

    // Parse object type
    let object_type = if mode == 3 && (type_flags & 128) != 0 {
        HitObjectType::Hold
    } else {
        HitObjectType::Circle
    };

    // Parse duration pour les hold notes
    let duration = if object_type == HitObjectType::Hold {
        // Skip hitSound
        start = end + 1;
        end = start;
        while end < line.len() && line.as_bytes()[end] != b',' {
            end += 1;
        }
        
        // Parse duration
        start = end + 1;
        end = start;
        while end < line.len() && line.as_bytes()[end] != b':' && line.as_bytes()[end] != b',' {
            end += 1;
        }
        Some(parse_int_fast(&line[start..end]))
    } else {
        None
    };

    Ok(HitObject {
        x,
        y,
        time,
        object_type,
        end_time: duration,
    })
}
