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

        // Pré-allouer la capacité pour éviter les reallocations
        self.hit_objects.reserve(50000);

        for line in content.lines() {
            // Skip les lignes vides et commentaires rapidement
            if line.is_empty() || line.as_bytes().get(0) == Some(&b'/') {
                continue;
            }

            // Check sections avec des comparaisons directes
            if line == "[General]" {
                in_hit_objects = false;
                continue;
            }

            if line == "[HitObjects]" {
                in_hit_objects = true;
                continue;
            }

            if !mode_found && line.starts_with("Mode:") {
                // Parse mode rapidement
                let mode_part = &line[5..];
                self.mode = mode_part.trim().parse().unwrap_or(0);
                mode_found = true;
                continue;
            }

            if in_hit_objects {
                self.hit_objects.push(parse_hit_object_line(line, self.mode)?);
            }
        }

        Ok(())
    }

    // Méthodes utilitaires
    pub fn get_circles(&self) -> Vec<&HitObject> {
        self.hit_objects.iter()
            .filter(|obj| matches!(obj.object_type, HitObjectType::Circle))
            .collect()
    }

    pub fn get_holds(&self) -> Vec<&HitObject> {
        self.hit_objects.iter()
            .filter(|obj| matches!(obj.object_type, HitObjectType::Hold))
            .collect()
    }

    pub fn get_objects_in_time_range(&self, start_time: i32, end_time: i32) -> Vec<&HitObject> {
        self.hit_objects.iter()
            .filter(|obj| obj.time >= start_time && obj.time <= end_time)
            .collect()
    }

    pub fn count_objects_by_type(&self) -> (usize, usize) {
        let mut circles = 0;
        let mut holds = 0;
        
        for obj in &self.hit_objects {
            match obj.object_type {
                HitObjectType::Circle => circles += 1,
                HitObjectType::Hold => holds += 1,
            }
        }
        
        (circles, holds)
    }

    pub fn get_hold_examples(&self, limit: usize) -> Vec<&HitObject> {
        self.hit_objects.iter()
            .filter(|obj| matches!(obj.object_type, HitObjectType::Hold))
            .take(limit)
            .collect()
    }
}
