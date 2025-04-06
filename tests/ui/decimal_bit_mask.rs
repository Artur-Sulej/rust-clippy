#![warn(clippy::decimal_bit_mask)]
#![allow(clippy::no_effect)]
#[rustfmt::skip]
fn main() {
    let mut x = 0;
    // one literal, bitwise op, decimal – bad
    x & 99;    //~ decimal_bit_mask
    x | 99;    //~ decimal_bit_mask
    x ^ 99;    //~ decimal_bit_mask
    x &= 99;   //~ decimal_bit_mask
    x |= 99;   //~ decimal_bit_mask
    x ^= 99;   //~ decimal_bit_mask

    // one literal, num op, decimal – good
    x += 99;
    x -= 99;
    x *= 99;
    x /= 99;
    x %= 99;

    // one literal, bitwise op, binary – good
    x & 0b1010;
    x | 0b1010;
    x ^ 0b1010;
    x &= 0b1010;
    x |= 0b1010;
    x ^= 0b1010;

    // one literal, num op, binary – good
    x += 0b1010;
    x -= 0b1010;
    x *= 0b1010;
    x /= 0b1010;
    x %= 0b1010;

    // two literals, bitwise op, decimal – bad
    0b1010 & 99;    //~ decimal_bit_mask
    0b1010 | 99;    //~ decimal_bit_mask
    0b1010 ^ 99;    //~ decimal_bit_mask

    // two literals, bitwise op, decimal – bad
    99 & 0b1010;
    99 | 0b1010;
    99 ^ 0b1010;

    // no literals – good
    let y = 0;
    x & y;
    x &= y;
    x + y;
    x += y;
}
