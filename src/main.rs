#![feature(if_let)]

extern crate rustbox;
extern crate water_weird;

use std::iter;
use std::collections::HashMap;
use water_weird::cell;

fn main() {
    let m = cell::diamond::random_diamond_map(5000, 50, 80, 50, 10);

    rustbox::init();
    rustbox::mode_256();

    cell::display(&m, &cell::diamond::StoneDiamondCell::Empty, 80, 50, 0, 10);

    rustbox::present();

    std::io::timer::sleep(std::time::Duration::milliseconds(1000));

    rustbox::shutdown();
}
