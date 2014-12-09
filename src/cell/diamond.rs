use std::rand;
use std::collections::HashMap;
use super::DisplayCell;

/// # StoneDiamondCell
/// Slightly more complicated than the StoneOrNotCell, it can have a non-occluding diamond.
pub enum StoneDiamondCell {
    Empty,
    Stone,
    Diamond,
}

impl DisplayCell for StoneDiamondCell {
    fn foreground(&self) -> Option<u32> {
        match *self {
            StoneDiamondCell::Stone => Some('#' as u32),
            StoneDiamondCell::Diamond => Some(0x2666 as u32),
            StoneDiamondCell::Empty => None,
        }
    }
    fn floor(&self) -> Option<u32> {
        match *self {
            StoneDiamondCell::Stone => Some('.' as u32),
            StoneDiamondCell::Diamond => None,
            StoneDiamondCell::Empty => None,
        }
    }
    fn color_in_background(&self) -> Option<u16> {
        match *self {
            StoneDiamondCell::Stone => Some(243), //medium grey
            StoneDiamondCell::Diamond => Some(23), //Dark cyan
            StoneDiamondCell::Empty => None,
        }
    }
    fn color_in_foreground(&self) -> u16 {
        match *self {
            StoneDiamondCell::Stone => 254, //light grey
            StoneDiamondCell::Diamond => 51, //cyan
            _ => 0, //should never be used.
        }
    }
    fn occludes_background(&self) -> bool {
        match *self {
            StoneDiamondCell::Stone => true,
            StoneDiamondCell::Diamond => false,
            StoneDiamondCell::Empty => false,
        }
    }
}

pub fn random_diamond_map(numstones: uint, numdiamonds: uint, height: u8, width: u8, depth: u8) -> HashMap<(u8, u8, u8), StoneDiamondCell> {
    let mut m = HashMap::new();
    for _ in range(0u, numstones) {
        let k = (rand::random::<u8>() % height, rand::random::<u8>() % width, rand::random::<u8>() % depth);
        m.insert(k, StoneDiamondCell::Stone);
    };
    for _ in range(0u, numdiamonds) {
        let k = (rand::random::<u8>() % height, rand::random::<u8>() % width, rand::random::<u8>() % depth);
        m.insert(k, StoneDiamondCell::Diamond);
    };
    m
}


