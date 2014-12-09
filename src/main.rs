#![feature(if_let)]

extern crate rustbox;

use std::iter;
use std::collections::HashMap;

trait DisplayCell {
    fn foreground(&self) -> Option<u32>;
    fn floor(&self) -> Option<u32>;
    fn color_in_background(&self) -> Option<u16>;
    fn color_in_foreground(&self) -> u16;
    fn occludes_background(&self) -> bool;
}

enum StoneOrNotCell {
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

fn display<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, topz: u8, width: u8, height: u8) {
    for x in range(0, width) {
        for y in range(0, height) {
            let (display_char, display_fore, display_back) = column_repr(map, default, x, y, topz);
            rustbox::change_cell(x as uint, y as uint, display_char, display_fore, display_back);
        }
    }
}
    
fn column_repr<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, x: u8, y: u8, topz: u8) -> (u32, u16, u16) {
    let (ch, fore) = column_fore(map, default, x, y, topz);
    let back = column_back(map, default, x, y, topz);
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

fn column_back<C: DisplayCell>(map: &HashMap<(u8, u8, u8), C>, default: &C, x: u8, y: u8, topz: u8) -> u16 {
    let depthlimit = 255u8;

    let firstcell = match map.get(&(x, y, topz)) {
        Some(c) => c,
        None => default
    };
    if !firstcell.occludes_background() {
        for z in iter::range(topz+1, depthlimit) {
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
        return 0u16;
    };

    0u16
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::{
        column_fore,
        column_back,
        column_repr,
        StoneOrNotCell
    };

    fn setupmap() -> HashMap<(u8, u8, u8), StoneOrNotCell> {
        let mut m = HashMap::new();
        m.insert((0, 0, 0), StoneOrNotCell::Stone);
        m.insert((1, 0, 1), StoneOrNotCell::Stone);
        m.insert((2, 0, 5), StoneOrNotCell::Stone);
        m
    }

    #[test]
    fn test_column_fore() {
        let m = setupmap();
        assert_eq!((35, 254), column_fore(&m, &StoneOrNotCell::Empty, 0, 0, 0));
        assert_eq!((46, 254), column_fore(&m, &StoneOrNotCell::Empty, 1, 0, 0));
        assert_eq!((32, 7), column_fore(&m, &StoneOrNotCell::Empty, 2, 0, 0));
        assert_eq!((32, 7), column_fore(&m, &StoneOrNotCell::Empty, 3, 0, 0));
    }

    #[test]
    fn test_column_back() {
        let m = setupmap();
        assert_eq!(0, column_back(&m, &StoneOrNotCell::Empty, 0, 0, 0));
        assert_eq!(243, column_back(&m, &StoneOrNotCell::Empty, 1, 0, 0));
        assert_eq!(243, column_back(&m, &StoneOrNotCell::Empty, 2, 0, 0));
        assert_eq!(0, column_back(&m, &StoneOrNotCell::Empty, 3, 0, 0));
    }

    #[test]
    fn test_column_repr() {
        let m = setupmap();
        assert_eq!((35, 254, 0), column_repr(&m, &StoneOrNotCell::Empty, 0, 0, 0));
        assert_eq!((46, 254, 243), column_repr(&m, &StoneOrNotCell::Empty, 1, 0, 0));
        assert_eq!((32, 7, 243), column_repr(&m, &StoneOrNotCell::Empty, 2, 0, 0));
        assert_eq!((32, 7, 0), column_repr(&m, &StoneOrNotCell::Empty, 3, 0, 0));
    }
}

fn main() {
    let mut m = HashMap::new();
    m.insert((0, 0, 0), StoneOrNotCell::Stone);
    m.insert((1, 0, 1), StoneOrNotCell::Stone);
    m.insert((2, 0, 5), StoneOrNotCell::Stone);

    rustbox::init();
    rustbox::mode_256();

    display(&m, &StoneOrNotCell::Empty, 0, 5, 5);

    rustbox::present();

    std::io::timer::sleep(std::time::Duration::milliseconds(1000));

    rustbox::shutdown();
}
