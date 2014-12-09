extern crate rustbox;

use std::iter;
use std::collections::HashMap;

pub trait DisplayCell {
    fn foreground(&self) -> Option<u32>;
    fn floor(&self) -> Option<u32>;
    fn color_in_background(&self) -> Option<u16>;
    fn color_in_foreground(&self) -> u16;
    fn occludes_background(&self) -> bool;
}

pub mod stone;
pub mod diamond;
pub mod weird;

pub fn display<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, width: u8, height: u8, topz: u8, bottomz: u8) {
    for x in range(0, width) {
        for y in range(0, height) {
            let (display_char, display_fore, display_back) = column_repr(map, default, x, y, topz, bottomz);
            rustbox::change_cell(x as uint, y as uint, display_char, display_fore, display_back);
        }
    }
}
    
fn column_repr<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, x: u8, y: u8, topz: u8, bottomz: u8) -> (u32, u16, u16) {
    let (ch, fore) = column_fore(map, default, x, y, topz);
    let back = column_back(map, default, x, y, topz, bottomz);
    (ch, fore, back)
}

fn column_fore<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, x: u8, y: u8, topz: u8) -> (u32, u16) {
    let firstcell = match map.get(&(x, y, topz)) {
        Some(c) => c,
        None => default
    };
    match firstcell.foreground() {
        Some(ch) => (ch, firstcell.color_in_foreground()),
        None => {
            let secondcell = match map.get(&(x, y, topz+1)) {
                Some(c) => c,
                None => default
            };
            match secondcell.floor() {
                Some(ch) => (ch, secondcell.color_in_foreground()),
                None => (' ' as u32, 7u16)
            }
        }
    }
}

fn column_back<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, x: u8, y: u8, topz: u8, bottomz: u8) -> u16 {
    let firstcell = match map.get(&(x, y, topz)) {
        Some(c) => c,
        None => default
    };
    if !firstcell.occludes_background() {
        for z in iter::range(topz+1, bottomz+1) {
            let cell = match map.get(&(x, y, z)) {
                Some(c) => c,
                None => default
            };
            match cell.color_in_background() {
                Some(color) => { return color; },
                None => { continue; }
            };
        }
    } else {
        return match firstcell.color_in_background() {
            Some(color) => { color },
            None => 0u16
        };
    };

    0u16
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::diamond::StoneDiamondCell;
    use super::{
        column_fore,
        column_back,
        column_repr,
    };

    fn setupmap() -> HashMap<(u8, u8, u8), StoneDiamondCell> {
        let mut m = HashMap::new();
        m.insert((0, 0, 0), StoneDiamondCell::Stone); // Should be a grey #
        m.insert((1, 0, 1), StoneDiamondCell::Stone); // Should be a light grey .
        m.insert((2, 0, 5), StoneDiamondCell::Stone); // Should be a dark gray background
        m.insert((0, 0, 7), StoneDiamondCell::Diamond); // Should be occluded
        m.insert((1, 0, 7), StoneDiamondCell::Diamond); // Should be occluded
        m.insert((2, 0, 7), StoneDiamondCell::Diamond); // Should be occluded

        m.insert((0, 1, 0), StoneDiamondCell::Diamond); // Should be a cyan diamond, with black background
        m.insert((1, 1, 1), StoneDiamondCell::Diamond); // Should be a dark cyan background
        m.insert((2, 1, 0), StoneDiamondCell::Diamond); // Should be a cyan diamond,
        m.insert((2, 1, 1), StoneDiamondCell::Stone);   //   with a dark gray background
        m
    }

    #[test]
    fn test_column_fore() {
        let m = setupmap();
        assert_eq!((35, 254), column_fore(&m, &StoneDiamondCell::Empty, 0, 0, 0));
        assert_eq!((46, 254), column_fore(&m, &StoneDiamondCell::Empty, 1, 0, 0));
        assert_eq!((32, 7), column_fore(&m, &StoneDiamondCell::Empty, 2, 0, 0));
        assert_eq!((32, 7), column_fore(&m, &StoneDiamondCell::Empty, 3, 0, 0));
    }

    #[test]
    fn test_column_back() {
        let m = setupmap();
        assert_eq!(243, column_back(&m, &StoneDiamondCell::Empty, 0, 0, 0, 10));
        assert_eq!(243, column_back(&m, &StoneDiamondCell::Empty, 1, 0, 0, 10));
        assert_eq!(243, column_back(&m, &StoneDiamondCell::Empty, 2, 0, 0, 10));
        assert_eq!(0, column_back(&m, &StoneDiamondCell::Empty, 3, 0, 0, 10));
    }

    #[test]
    fn test_column_repr() {
        let m = setupmap();
        assert_eq!((35, 254, 243), column_repr(&m, &StoneDiamondCell::Empty, 0, 0, 0, 10));
        assert_eq!((46, 254, 243), column_repr(&m, &StoneDiamondCell::Empty, 1, 0, 0, 10));
        assert_eq!((32, 7, 243), column_repr(&m, &StoneDiamondCell::Empty, 2, 0, 0, 10));
        assert_eq!((32, 7, 0), column_repr(&m, &StoneDiamondCell::Empty, 3, 0, 0, 10));

        assert_eq!((0x2666, 51, 0), column_repr(&m, &StoneDiamondCell::Empty, 0, 1, 0, 10));
        assert_eq!((32, 7, 23), column_repr(&m, &StoneDiamondCell::Empty, 1, 1, 0, 10));
        assert_eq!((0x2666, 51, 243), column_repr(&m, &StoneDiamondCell::Empty, 2, 1, 0, 10));
    }
}


