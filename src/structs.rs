#[derive(Debug, Clone, PartialEq)]
pub enum HitObjectType {
    Circle,
    Hold,
}

#[derive(Debug, Clone)]
pub struct HitObject {
    pub x: i32,
    pub y: i32,
    pub time: i32,
    pub object_type: HitObjectType,
    pub end_time: Option<i32>, // Pour les hold notes
}

#[derive(Debug, Clone)]
pub struct OsuParser {
    pub mode: u8,
    pub hit_objects: Vec<HitObject>,
}
