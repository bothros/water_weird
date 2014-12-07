#![feature(if_let)]

extern crate rustbox;

use std::char;
use std::iter;

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

fn display<C: DisplayCell>(map: &[ &[ &[C] ] ]) { //3d array of cells, first x, then y, then z
    let mut char_set = false;

    for (horizontal_slice, x) in map.iter().zip(iter::count(0u, 1)) {
        for (column, y) in horizontal_slice.iter().zip(iter::count(0u, 1)) {
            let (display_char, display_fore, display_back) = column_repr(*column);
            rustbox::change_cell(x, y, display_char, display_fore, display_back);
        }
    }
}

fn column_repr<C: DisplayCell>(column: &[C]) -> (u32, u16, u16) {
    let (ch, fore) = column_fore(column);
    let back = column_back(column);
    (ch, fore, back)
}

fn column_fore<C: DisplayCell>(column: &[C]) -> (u32, u16) {
    match column[0].foreground() {
        Some(ch) => (ch, column[0].color_in_foreground()), 
        None => {
            match column[1].floor() {
                Some(ch) => (ch, column[1].color_in_foreground()),
                None => (' ' as u32, 0u16)
            }
        }
    }
}

fn column_back<C: DisplayCell>(column: &[C]) -> u16 {
    let mut coliter = column.iter();
    
    match coliter.next() {
        Some(firstcell) => {
            if !firstcell.occludes_background() {
                for cell in coliter {
                    match cell.color_in_background() {
                        Some(color) => { return color; },
                        None => {continue; }
                    }
                }
            } else {
                return 0u16;
            }
        },
        None => { return 0u16; }
    };

    0u16 //we shouldn't ever get here
}

fn stonemap<'a, 'b, 'c>(bitmap: &'a [ &'b [ &'c [u8] ] ]) -> &'a [ &'b [ &'c [StoneOrNotCell] ] ] {
    bitmap.iter().map( |horizontal_slice| {
        horizontal_slice.iter().map( |column| {
            column.iter().map( |bit| {
                match *bit {
                    1 => StoneOrNotCell::Stone,
                    0 => StoneOrNotCell::Empty,
                    _ => StoneOrNotCell::Empty
                }
            }).collect()
        }).collect()
    }).collect()
}

fn main() {
    let x = &[StoneOrNotCell::Empty, StoneOrNotCell::Stone];
    println!("{}", column_fore(x));
    println!("{}", column_back(x));
    println!("{}", column_repr(x));
}
