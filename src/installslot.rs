#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct InstallSlot {
    hour_idx_from: u8,
    hour_idx_to: u8,
}

fn get_weekday_from_hour_idx(hour_idx: u8) -> &'static str {
    match hour_idx / 24 {
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Saturday",
        6 => "Sunday",
        _ => panic!("hour_idx out of range"),
    }
}

fn get_hour_within_day_from_hour_idx(hour_idx: u8) -> u8 {
    hour_idx % 24
}

impl InstallSlot {
    fn create(from: u8, to: u8) -> InstallSlot {
        InstallSlot {
            hour_idx_from: from,
            hour_idx_to: to,
        }
    }

    fn from(from_day: u8, from_hour: u8, to_day: u8, to_hour: u8) -> InstallSlot {
        InstallSlot {
            hour_idx_from: from_day * 24 + from_hour,
            hour_idx_to: to_day * 24 + to_hour,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "from {} {} to {} {}",
            get_weekday_from_hour_idx(self.hour_idx_from),
            get_hour_within_day_from_hour_idx(self.hour_idx_from),
            get_weekday_from_hour_idx(self.hour_idx_to),
            get_hour_within_day_from_hour_idx(self.hour_idx_to)
        )
    }
}

pub fn print_slots(slots: &Vec<InstallSlot>) {
    for slot in slots {
        println!("{}", slot.to_string());
    }
}

pub fn from_hex_to_install_slots<F>(
    bitfield_week: &Vec<u8>,
    on_install_slot: F) where F : Fn(&InstallSlot) {

    let mut start_hour_week_idx: Option<u8> = None;
    let mut hours_idx: u8 = 0;
    let mut slot: InstallSlot;

    for byte_8_hours in bitfield_week {
        for i in 0..8 {
            let installation_enabled: bool = ( (0x80 >> i) & *byte_8_hours ) > 1;

            match start_hour_week_idx {
                None => {
                    if installation_enabled {
                        start_hour_week_idx = Some(hours_idx);
                    }
                }
                Some(start_idx) => {
                    if ! installation_enabled {
                        slot.hour_idx_from = start_idx;
                        slot.hour_idx_to = hours_idx;
                    }
                }
            }

            hours_idx = hours_idx + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::installslot::{from_hex_to_install_slots, InstallSlot};

    const WEEK_BYTE_LEN: usize = 24 * 7 / 8;

    fn get_slots(pattern: &Vec<u8>) -> Vec<InstallSlot> {

        let mut result: Vec<InstallSlot> = vec![];

        from_hex_to_install_slots(
            pattern,
            |slot| {
                result.push(slot.clone());
            });

        result
    }

    #[test]
    fn no_install_slots() {
        let mut pattern: Vec<u8> = vec![0; WEEK_BYTE_LEN];
        let expected: Vec<InstallSlot> = vec![];
        assert_eq!(expected, get_slots(&pattern));
    }

    #[test]
    fn monday_from_0_to_1() {
        let mut pattern: Vec<u8> = vec![0; WEEK_BYTE_LEN];
        println!("pattern len: {}", pattern.len());
        pattern[0] = 0x80;

        let expected = vec![InstallSlot::from(0, 0, 0, 1)];
        assert_eq!(expected, get_slots(&pattern));
    }

    #[test]
    fn tuesday_from_midnight_to_4() {
        let mut pattern: Vec<u8> = vec![0; WEEK_BYTE_LEN];
        let dayoffset_byte = 24 / 8;
        pattern[dayoffset_byte * 1] = 0xF0;

        let expected = vec![InstallSlot::from(1, 0, 1, 4)];

        assert_eq!(expected, get_slots(&pattern));
    }
}
