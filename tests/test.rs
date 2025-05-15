use bitfi::{bitfield, BitField};

#[cfg(test)]
extern crate std;

bitfield! {
    TestBf = u16 {
        on: 2;
        love : 0 ..= 1;
    }

    TestB2 = u32 {
        love : 0 ..= 1;
        love2 : 2 ..= 4;
        war: 5 ..= 8 [mut = false];
    }
}

#[test]
fn test_simple() {
    let mut bf = TestBf::default();

    for i in 0..16 {
        assert!(!bf.get_bit(i), "Joder {i} {:b}", bf.0);
        bf.set_bit(i);
        assert!(bf.get_bit(i), "Joder {i} {:b}", bf.0);
    }
}

#[test]
fn test_ranges() {
    let mut bf = TestB2::new(0);

    assert_eq!(bf.get_love(), 0);
    bf.set_love(0b11);
    assert_eq!(bf.get_love(), 0b11);

    bf.set_love(0b10);
    assert_eq!(bf.get_love(), 0b10);

    bf.set_love(0b01);
    assert_eq!(bf.get_love(), 0b01);

    bf.set_love2(0b101);

    assert_eq!(bf.get_inner(), 0b10101, "{:b} {:b}", bf.get_inner(), 0b10101);
}

#[test]
fn nums() {
    let mut n = 0;

    n.set_bit_range(0..=1, 0b11);
    n.set_bit_range(2..=3, 0b10);

    assert_eq!(n, 0b1011, "{n:b} {:b}", 0b1011);

    let br = n.get_bit_range(0..=1);
    assert_eq!(br, 0b11, "{br:b} {:b}", 0b11);

    let br = n.get_bit_range(2..=3);
    assert_eq!(br, 0b10, "{br:b} {:b}", 0b10);
}
