#![warn(clippy::decimal_bit_mask)]
#![allow(clippy::no_effect)]
#[rustfmt::skip]
fn main() {
    let mut x = 0;
    x & 99;    //~ decimal_bit_mask
    x | 99;    //~ decimal_bit_mask
    x ^ 99;    //~ decimal_bit_mask
    x &= 99;   //~ decimal_bit_mask
    x |= 99;   //~ decimal_bit_mask
    x ^= 99;   //~ decimal_bit_mask

    x += 99;
    x -= 99;
    x *= 99;
    x /= 99;
    x %= 99;

    x & 0b1010;
    x | 0b1010;
    x ^ 0b1010;
    x &= 0b1010;
    x |= 0b1010;
    x ^= 0b1010;

    let mut y = 0;
    x ^= y;

}
