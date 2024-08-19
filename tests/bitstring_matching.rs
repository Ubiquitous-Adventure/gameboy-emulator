use gameboy_emulator::bits;

#[test]
fn simple_bitstring_matching() {
    for i in 0..=u8::MAX {
        match i {
            bits!(00000101) => assert_eq!(i, 5),
            bits!(__100010) => assert_eq!(i & 0b00111111, 0b00100010),
            _ => {}
        }
    }
}

#[test]
fn mixed_bitstring_matching() {
    for i in 0..=u8::MAX {
        match i {
            bits!(0000010_) | bits!(_0000_01) => {
                assert!([0b00000001, 0b00000100, 0b00000101, 0b10000001, 0b10000101].contains(&i))
            }
            bits!(__11____) => {
                assert_eq!(i & 0b00110000, 0b00110000)
            }
            _ => {}
        }
    }
}

#[test]
fn bitstring_matching_large_suffix() {
    for i in 0..=u8::MAX {
        match i {
            bits!(01______) => {
                assert!((0b01000000..0b10000000).contains(&i));
            }
            _ => {
                assert!(!(0b01000000..0b10000000).contains(&i));
            }
        }
    }
}
