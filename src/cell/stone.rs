use std::rand;
use std::collections::HashMap;
use super::DisplayCell;

pub enum StoneOrNotCell {
    Empty,
    Stone,
}

impl DisplayCell for StoneOrNotCell {
    fn foreground(&self) -> Option<u32> {
        match *self {
            StoneOrNotCell::Stone => Some('#' as u32),
            StoneOrNotCell::Empty => None
        }
    }
    fn floor(&self) -> Option<u32> {
        match *self {
            StoneOrNotCell::Stone => Some('.' as u32),
            StoneOrNotCell::Empty => None
        }
    }
    fn color_in_background(&self) -> Option<u16> {
        match *self {
            StoneOrNotCell::Stone => Some(243), //medium grey
            StoneOrNotCell::Empty => None
        }
    }
    fn color_in_foreground(&self) -> u16 {
        match *self {
            StoneOrNotCell::Stone => 254, //light grey
            _ => 0, //should never be used.
        }
    }
    fn occludes_background(&self) -> bool {
        match *self {
            StoneOrNotCell::Stone => true,
            StoneOrNotCell::Empty => false
        }
    }
}

pub fn random_stone_map(numstones: uint, height: u8, width: u8, depth: u8) -> HashMap<(u8, u8, u8), StoneOrNotCell> {
    let mut m = HashMap::new();
    for _ in range(0u, numstones) {
        let k = (rand::random::<u8>() % height, rand::random::<u8>() % width, rand::random::<u8>() % depth);
        m.insert(k, StoneOrNotCell::Stone);
    };
    m
}


