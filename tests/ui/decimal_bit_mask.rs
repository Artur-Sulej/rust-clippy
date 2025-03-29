#![warn(clippy::decimal_bit_mask)]
#![allow(clippy::no_effect)]

fn main() {
    let mut x = 0;
    let mut y = 0;
    x & 128; //~ decimal_bit_mask
    x | 1024; //~ decimal_bit_mask
    x ^ 1024; //~ decimal_bit_mask
    x &= 1; //~ decimal_bit_mask
    x |= 2; //~ decimal_bit_mask
    x ^= 3; //~ decimal_bit_mask

    x += 3;

    x & 0b1100;
    x | 0b1100;
    x ^ 0b1100;
    x &= 0b1100;
    x |= 0b1100;
    x ^= 0b1100;

    x ^= y;
}
