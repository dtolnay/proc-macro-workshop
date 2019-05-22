#![feature(test)]

//! In this benchmark we compare the macro generated code for
//! setters and getters to some hand-written code for the same
//! data structure.
//!
//! We do a performance analysis for the getter and setter of
//! all fields of both structs.
//!
//! Also we test here that our hand-written code and the macro
//! generated code actually are semantically equivalent.
//! This allows us to further enhance the hand-written code
//! and to eventually come up with new optimization tricks
//! for the macro generated code while staying correct.

extern crate test;

use test::{black_box, Bencher};

use bitfield::*;

/// This generates code by the macros that we are going to test.
///
/// For every field a getter `get_*` and a setter `set_*` is generated
/// where `*` is the name of the field.
///
/// Note that this tests the following cases:
///
/// - `a`: Spans 2 bytes where the first byte is used fully and the
///        second byte stores only one of its bits.
/// - `b`: Fits into one byte but doesn't reach the bounds on either side.
/// - `c`: Spans across 3 bytes in total and uses only 1 bit and 4 bits in
///        the respective first and last byte.
/// - `d`: Spans 3 whole bytes in total.
///
/// More cases could be missing and might be added in the future.
#[bitfield]
pub struct MacroStruct {
    a: B9,
    b: B6,
    c: B13,
    d: B4,
    e: B32,
}

/// This is the hand-written part that the macro generated getters
/// and setters are compared against.
///
/// We try to encode the handwritten setters and getters as good as
/// we can while trying to stay within reasonable bounds of readability.
///
/// This code should perform as good as the macro generated code and vice versa.
pub struct HandStruct {
    data: [u8; 8],
}

impl HandStruct {
    /// Creates a new hand-written struct initialized with all zeros.
    pub fn new() -> Self {
        Self { data: [0; 8] }
    }

    /// Returns the value of `a`.
    pub fn get_a(&self) -> u16 {
        u16::from_le_bytes([
            self.data[0],
            self.data[1] & 0x1,
        ])
    }

    /// Sets the value of `a`.
    pub fn set_a(&mut self, new_val: u16) {
        assert!(new_val < (0x1 << 9));
        let [ls, ms] = new_val.to_le_bytes();
        self.data[0] = ls;
        self.data[1] = (self.data[1] & (!0x1)) | (ms & 0x1);
    }

    /// Returns the value of `b`.
    pub fn get_b(&self) -> u8 {
        (self.data[1] >> 1) & 0b0011_1111
    }

    /// Sets the value of `b`.
    pub fn set_b(&mut self, new_val: u8) {
        assert!(new_val < (0x1 << 6));
        self.data[1] = (self.data[1] & 0x81) | (new_val << 1);
    }

    /// Returns the value of `c`.
    pub fn get_c(&self) -> u16 {
        let mut res = 0;
        res |= (self.data[1] >> 7) as u16;
        res |= (self.data[2] as u16) << 1;
        res |= (((self.data[3] & 0b1111) as u16) << 9) as u16;
        res
    }

    /// Sets the value of `c`.
    pub fn set_c(&mut self, new_val: u16) {
        assert!(new_val < (0x1 << 13));
        self.data[1] = (self.data[1] & !0x80) | (((new_val & 0x1) << 7) as u8);
        self.data[2] = ((new_val >> 1) & 0xFF) as u8;
        self.data[3] = (self.data[3] & !0x0F) | (((new_val >> 9) & 0x0F) as u8);
    }

    /// Returns the value of `d`.
    pub fn get_d(&self) -> u8 {
        self.data[3] >> 4
    }

    /// Sets the value of `d`.
    pub fn set_d(&mut self, new_val: u8) {
        assert!(new_val < (0x1 << 4));
        self.data[3] = (self.data[3] & 0xF0) | ((new_val & 0x0F) << 4)
    }

    /// Returns the value of `e`.
    pub fn get_e(&self) -> u32 {
        u32::from_le_bytes([self.data[4], self.data[5], self.data[6], self.data[7]])
    }

    /// Sets the value of `e`.
    pub fn set_e(&mut self, new_val: u32) {
        assert!((new_val as u64) < (0x1_u64 << 32));
        let le_bytes = new_val.to_le_bytes();
        for (n, byte) in le_bytes.iter().enumerate() {
            self.data[n + 4] = *byte;
        }
    }
}

const REPETITIONS: usize = 300;

macro_rules! gen_access_bench_for {
    ( $( ($get_name:ident, $set_name:ident) ),* ) => {
        mod generated {
            use super::*;

            $(
                #[bench]
                fn $get_name(bencher: &mut Bencher) {
                    let data = MacroStruct::new();
                    bencher.iter(|| {
                        for _ in 0..REPETITIONS {
                            black_box(data.$get_name());
                        }
                    });
                }

                #[bench]
                fn $set_name(bencher: &mut Bencher) {
                    let mut data = MacroStruct::new();
                    bencher.iter(|| {
                        for _ in 0..REPETITIONS {
                            black_box(data.$set_name(black_box(1)));
                        }
                    });
                }
            )*
        }

        mod handwritten {
            use super::*;

            $(
                #[bench]
                fn $get_name(bencher: &mut Bencher) {
                    let data = HandStruct::new();
                    bencher.iter(|| {
                        for _ in 0..REPETITIONS {
                            black_box(data.$get_name());
                        }
                    });
                }

                #[bench]
                fn $set_name(bencher: &mut Bencher) {
                    let mut data = HandStruct::new();
                    bencher.iter(|| {
                        for _ in 0..REPETITIONS {
                            black_box(data.$set_name(black_box(1)));
                        }
                    });
                }
            )*
        }
    }
}

gen_access_bench_for!(
    (get_a, set_a),
    (get_b, set_b),
    (get_c, set_c),
    (get_d, set_d),
    (get_e, set_e)
);
