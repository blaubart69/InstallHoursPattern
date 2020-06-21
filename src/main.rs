use crate::installslot::InstallSlot;
use std::num::ParseIntError;

mod installslot;

const WEEK_BYTE_LEN : usize = 24*7/8;

fn main() {

    let inst_hours_pattern : Vec<u8> = vec![
    //              111111    11112222
    //01234567    89012345    67890123
    0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00001111,
    0b11110000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000];

    let printOneSlot = | slot:&InstallSlot| {
        println!("{}", slot.to_string());
    };

    installslot::from_hex_to_install_slots(&inst_hours_pattern, printOneSlot);

}


