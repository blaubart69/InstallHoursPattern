use crate::installslot::InstallSlot;

mod installslot;

const WEEK_BYTE_LEN : usize = 24*7/8;

fn main() {
    let mut pattern : Vec<u8> = vec![0; WEEK_BYTE_LEN];
    pattern[0] = 0x80;
    let dayoffset_byte = 24 / 8;
    pattern[ dayoffset_byte * 1] = 0xF0;

    let printOneSlot = | slot:&InstallSlot| {
        println!("{}", slot.to_string());
    };

    installslot::from_hex_to_install_slots(&pattern, printOneSlot);

}


