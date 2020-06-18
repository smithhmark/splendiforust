mod gems;

use crate::gems::{GemPool, GemType};

#[allow(dead_code)]
struct Improvement {
    name: String,
    flavor: GemType,
    bonus: usize,
    cost: GemPool,
}

#[allow(dead_code)]
struct PlayerPosition {
    gems: GemPool,
    improvements: Vec<Improvement>,
}

#[allow(dead_code)]
struct TablePosition {
    gems: GemPool,
    improvements1: Vec<Improvement>,
    players: Vec<PlayerPosition>,
}