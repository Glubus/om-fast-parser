use std::path::Path;
use std::time::Instant;

use super_fast_parser::OsuParser;

fn main() {
    let mut parser = OsuParser::new();
    
    let start = Instant::now();
    let parse_result = parser.parse_file(Path::new("assets/smk.osu"));
    let elapsed = start.elapsed();
    
    match parse_result {
        Ok(()) => {
            println!("Parsed {} hit objects in {:?}", parser.hit_objects.len(), elapsed);
            println!("Mode: {}", parser.mode);
            
            // Utiliser la nouvelle méthode pour compter les objets
            let (circles, holds) = parser.count_objects_by_type();
            println!("Circles: {}", circles);
            println!("Holds: {}", holds);
            
            // Afficher quelques exemples
            for (i, obj) in parser.hit_objects.iter().take(5).enumerate() {
                println!("Object {}: {:?} at time {}", i, obj.object_type, obj.time);
            }
            
            // Afficher quelques hold notes avec la nouvelle méthode
            println!("\nHold notes examples:");
            for obj in parser.get_hold_examples(5) {
                println!("Hold at time: {}, duration: {:?}", obj.time, obj.end_time);
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
        }
    }
}