#![feature(if_let)]

extern crate rustbox;
extern crate water_weird;

use water_weird::cell;

fn main() {
    let m = cell::diamond::random_diamond_map(5000, 50, 80, 50, 10);
    cell::view(&m, &cell::diamond::StoneDiamondCell::Empty, 80, 50, 10, Some(2));
}
