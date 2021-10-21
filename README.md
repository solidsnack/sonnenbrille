# Sonnenbrille

*["Son­nen­bril­le, die -- Brille mit dunkel getönten Gläsern, die die
Augen vor zu starker Helligkeit des Sonnenlichts schützen soll"][sb]*
-- Duden

[sb]: https://www.duden.de/rechtschreibung/Sonnenbrille

*"Sunglasses, feminine -- Glasses with dark-toned lenses, that protect
the eyes from bright sunlight"*

There are many articles, references and online resources for the Cyclic
Redundancy Check, but I was surprised and greatly helped by the clarity
and comprehensiveness of [Sunshine 2K][s2k]'s
*[Understanding and implementing CRC (Cyclic Redundancy Check)
calculation][U]*. Along with the author's [online implementation][JS],
this illuminating article made it possible for me to understand,
implement and test an 8-bit CRC calculator in Rust.

[s2k]: http://www.sunshine2k.de/
[U]: http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html
[JS]: http://www.sunshine2k.de/coding/javascript/crc/crc_js.html

```rs
extern crate sonnenbrille;
use sonnenbrille::CRC8;

fn crc8(num: u32): u8 {
    let calculator = CRC8::default();
    return calculator.of(&num.to_be_bytes(), 0x00);
}

fn main() {
    let num: u32 = 0x31313233;
    let calculator = CRC8::default();
    let checksum = calculator.of(&num.to_be_bytes(), 0x00);
    assert_eq!(checksum, 0x7F);
}
```