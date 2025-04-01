use std::collections::HashMap;
use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u16)] // Use a 16-bit underlying storage for all your bitfields
pub struct CommandFlags {
    type_: B2,    // 2 bits
    cmd: B3,      // 3 bits
    type1: B1,    // 1 bit
    type2: B1,    // 1 bit
    error: B2,    // 2 bits
    is_div_by_zero: B1, // 1 bit
    #[skip]      // Skip the remaining bits (if any) or you can pack another value here.
    __: B6,      // to round up to 16 bits (total now is 2+3+1+1+2+1+5 = 15+? Actually, we need exactly 10 bits, but we choose 16-bit storage)
}

pub struct CommandCall {
    flags: CommandFlags, // uses 2 bytes
    param1: i32,         // 4 bytes
    param2: i32,         // 4 bytes
    value: i32,         // 4 bytes
    map: Box< Vec<i32> >// pointer to a heap-allocated HashMap
}

fn main() {
    println!("Size of CommandCall: {} bytes", std::mem::size_of::<CommandCall> ());
}
