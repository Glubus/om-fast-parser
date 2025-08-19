use std::fs;
use std::path::Path;
use crate::structs::{OsuParser, HitObject, HitObjectType};
use crate::helpers::parse_hit_object_line;

impl OsuParser {
    pub fn new() -> Self {
        Self {
            mode: 0,
            hit_objects: Vec::new(),
        }
    }

    pub fn parse_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        self.parse_content(&content)
    }

    pub fn parse_content(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut in_hit_objects = false;
        let mut mode_found = false;

        self.hit_objects.reserve(50000);

        for line in content.lines() {
            let bytes = line.as_bytes();
            if bytes.is_empty() || bytes[0] == b'/' { continue; }

            match bytes {
                b"[General]" => { in_hit_objects = false; continue; }
                b"[HitObjects]" => { in_hit_objects = true; continue; }
                _ => {}
            }

            if !mode_found && bytes.starts_with(b"Mode:") {
                self.mode = crate::helpers::parse_int_fast(&bytes[5..]) as u8;
                mode_found = true;
                continue;
            }

            if in_hit_objects {
                let hit_object = parse_hit_object_line(bytes, self.mode);
                self.hit_objects.push(hit_object);
            }
        }

        Ok(())
    }

    pub fn get_circles(&self) -> Vec<&HitObject> {
        self.hit_objects.iter().filter(|obj| matches!(obj.object_type, HitObjectType::Circle)).collect()
    }

    pub fn get_holds(&self) -> Vec<&HitObject> {
        self.hit_objects.iter().filter(|obj| matches!(obj.object_type, HitObjectType::Hold)).collect()
    }

    pub fn get_objects_in_time_range(&self, start_time: i32, end_time: i32) -> Vec<&HitObject> {
        self.hit_objects.iter().filter(|obj| obj.time >= start_time && obj.time <= end_time).collect()
    }

    pub fn count_objects_by_type(&self) -> (usize, usize) {
        self.hit_objects.iter().fold((0, 0), |(c, h), obj| match obj.object_type { HitObjectType::Circle => (c + 1, h), HitObjectType::Hold => (c, h + 1) })
    }

    pub fn get_hold_examples(&self, limit: usize) -> Vec<&HitObject> {
        self.hit_objects.iter().filter(|obj| matches!(obj.object_type, HitObjectType::Hold)).take(limit).collect()
    }

    pub fn get_objects_by_time_sorted(&self) -> Vec<&HitObject> {
        let mut objects: Vec<&HitObject> = self.hit_objects.iter().collect();
        objects.sort_by_key(|obj| obj.time);
        objects
    }

    pub fn get_density_analysis(&self, time_window: i32) -> Vec<(i32, usize)> {
        let mut density_map = std::collections::HashMap::new();
        for obj in &self.hit_objects {
            let window_start = obj.time / time_window * time_window;
            *density_map.entry(window_start).or_insert(0) += 1;
        }
        let mut result: Vec<_> = density_map.into_iter().collect();
        result.sort_by_key(|(time, _)| *time);
        result
    }
}

 
