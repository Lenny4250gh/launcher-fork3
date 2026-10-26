//! The light bar colour of a DualSense (docs/DESIGN.md 6.10), set from the Controllers page.
//!
//! This is the one place the launcher writes to a pad's HID output, and only because the player
//! asked on that page, only for a DualSense, and only while no game runs: a game owns the pad
//! while it is in front, and the next report it sends replaces the colour anyway. The caller
//! checks "no game"; [`set`] does the writing. The report layouts are generic device facts from
//! the public sources of the Linux kernel (`drivers/hid/hid-playstation.c`, the
//! `dualsense_output_report_*` structs and `dualsense_output_worker`) and SDL
//! (`src/joystick/hidapi/SDL_hidapi_ps5.c`, `DS5EffectsState_t`).
//!
//! The common part of the output report is 47 bytes: `valid_flag0` (0), `valid_flag1` (1),
//! rumble and audio fields up to `valid_flag2` (38), `lightbar_setup` (41) and the colour at 44,
//! 45 and 46. `valid_flag1` bit 2 says the colour is meant; `valid_flag2` bit 1 with
//! `lightbar_setup` 2 turns off the blue fade-out a pad shows after it has been paired. Only the
//! lightbar bits are set, so rumble and the player lamps are left as they are.
//!
//! | Link | Report id | Length | Common part starts at |
//! |---|---|---|---|
//! | USB | 0x02 | 63 | 1 |
//! | Bluetooth | 0x31 | 78 | 3 (sequence in the high nibble of byte 1, tag 0x10 at byte 2, CRC-32 in the last four) |
//!
//! The Bluetooth CRC is CRC-32 over a 0xA2 byte and the report without its last four bytes.

#![cfg_attr(not(any(windows, target_os = "linux")), allow(dead_code))]

use crate::ui::pad_hub::Link;
use crate::ui::ps_button::crc32;

const USB_ID: u8 = 0x02;
const USB_LEN: usize = 63;
const BT_ID: u8 = 0x31;
const BT_LEN: usize = 78;
const BT_TAG: u8 = 0x10;
const BT_CRC_SEED: u8 = 0xA2;

/// Offsets inside the common part.
const FLAG1: usize = 1;
const FLAG2: usize = 38;
const SETUP: usize = 41;
const RED: usize = 44;

const FLAG1_LIGHTBAR: u8 = 1 << 2;
const FLAG2_LIGHTBAR_SETUP: u8 = 1 << 1;
const SETUP_LIGHT_OUT: u8 = 1 << 1;

/// The colours the page offers, as `(name, RRGGBB)`. The first is the pad's own.
pub const COLOURS: [(&str, [u8; 3]); 7] = [
    ("Blue", [0x00, 0x30, 0xFF]),
    ("Red", [0xFF, 0x1A, 0x1A]),
    ("Green", [0x12, 0xD6, 0x4A]),
    ("Purple", [0x9A, 0x3C, 0xFF]),
    ("Orange", [0xFF, 0x7A, 0x00]),
    ("White", [0xFF, 0xFF, 0xFF]),
    ("Off", [0x00, 0x00, 0x00]),
];

/// `RRGGBB` as the preferences keep it.
pub fn hex(rgb: [u8; 3]) -> String {
    format!("{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

pub fn parse(text: &str) -> Option<[u8; 3]> {
    if text.len() != 6 || !text.is_ascii() {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(&text[i..i + 2], 16).ok();
    Some([byte(0)?, byte(2)?, byte(4)?])
}

/// The index in [`COLOURS`] of the colour the preferences hold, or of the pad's own.
pub fn index_of(stored: Option<&str>) -> usize {
    stored
        .and_then(parse)
        .and_then(|rgb| COLOURS.iter().position(|(_, c)| *c == rgb))
        .unwrap_or(0)
}

/// A report that makes the light bar `rgb`, for a pad on `link`. `seq` is the Bluetooth
/// sequence number (0 to 15). With `setup` the report also turns off the pairing fade-out.
pub fn report(link: Link, rgb: [u8; 3], setup: bool, seq: u8) -> Vec<u8> {
    let (mut report, start) = match link {
        Link::Bluetooth => {
            let mut r = vec![0u8; BT_LEN];
            r[0] = BT_ID;
            r[1] = (seq & 0x0F) << 4;
            r[2] = BT_TAG;
            (r, 3)
        }
        _ => {
            let mut r = vec![0u8; USB_LEN];
            r[0] = USB_ID;
            (r, 1)
        }
    };
    if setup {
        report[start + FLAG2] |= FLAG2_LIGHTBAR_SETUP;
        report[start + SETUP] = SETUP_LIGHT_OUT;
    } else {
        report[start + FLAG1] |= FLAG1_LIGHTBAR;
        report[start + RED..start + RED + 3].copy_from_slice(&rgb);
    }
    if link == Link::Bluetooth {
        let split = BT_LEN - 4;
        let crc = crc32(crc32(0, &[BT_CRC_SEED]), &report[..split]);
        report[split..].copy_from_slice(&crc.to_le_bytes());
    }
    report
}

/// Sets the light bar of every DualSense the system lists. Returns how many took it. The
/// caller has checked that no game runs.
#[cfg(any(windows, target_os = "linux"))]
pub fn set(rgb: [u8; 3]) -> Result<usize, String> {
    use hidapi::{BusType, HidApi};
    const SONY: u16 = 0x054C;
    let api = HidApi::new().map_err(|e| e.to_string())?;
    let mut done = 0;
    let mut last_error = None;
    for info in api.device_list() {
        if info.vendor_id() != SONY || !matches!(info.product_id(), 0x0CE6 | 0x0DF2) {
            continue;
        }
        let link = match info.bus_type() {
            BusType::Bluetooth => Link::Bluetooth,
            _ => Link::Usb,
        };
        let device = match info.open_device(&api) {
            Ok(device) => device,
            Err(err) => {
                last_error = Some(err.to_string());
                continue;
            }
        };
        let sent = device
            .write(&report(link, rgb, true, 0))
            .and_then(|_| device.write(&report(link, rgb, false, 1)));
        match sent {
            Ok(_) => done += 1,
            Err(err) => last_error = Some(err.to_string()),
        }
    }
    match (done, last_error) {
        (0, Some(err)) => Err(err),
        _ => Ok(done),
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn set(_rgb: [u8; 3]) -> Result<usize, String> {
    Err("not supported on this system".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_usb_report_has_the_colour_where_the_kernel_puts_it() {
        let r = report(Link::Usb, [1, 2, 3], false, 0);
        assert_eq!(r.len(), 63);
        assert_eq!(r[0], 0x02);
        // Common part starts at 1: flag1 at 2, colour at 45, 46 and 47.
        assert_eq!(r[2], 0x04);
        assert_eq!(&r[45..48], [1, 2, 3]);
        // Nothing else is switched on: no rumble, no audio, no player lamps.
        assert_eq!(r[1], 0);
        assert_eq!(r[39], 0);
        assert!(r[48..].iter().all(|b| *b == 0));
    }

    #[test]
    fn the_setup_report_turns_the_fade_out_off_and_sets_no_colour() {
        let r = report(Link::Usb, [9, 9, 9], true, 0);
        assert_eq!(r[1 + 38], 0x02);
        assert_eq!(r[1 + 41], 0x02);
        assert_eq!(r[2], 0);
        assert_eq!(&r[45..48], [0, 0, 0]);
    }

    #[test]
    fn a_bluetooth_report_is_signed_and_numbered() {
        let r = report(Link::Bluetooth, [10, 20, 30], false, 5);
        assert_eq!(r.len(), 78);
        assert_eq!(r[0], 0x31);
        assert_eq!(r[1], 0x50, "sequence in the high nibble");
        assert_eq!(r[2], 0x10);
        // Common part starts at 3: flag1 at 4, colour at 47, 48 and 49.
        assert_eq!(r[4], 0x04);
        assert_eq!(&r[47..50], [10, 20, 30]);
        let crc = u32::from_le_bytes([r[74], r[75], r[76], r[77]]);
        assert_eq!(crc, crc32(crc32(0, &[0xA2]), &r[..74]));
        // A different colour is a different signature.
        let other = report(Link::Bluetooth, [10, 20, 31], false, 5);
        assert_ne!(&r[74..], &other[74..]);
    }

    #[test]
    fn colours_round_trip_through_the_preferences() {
        for (i, (_, rgb)) in COLOURS.iter().enumerate() {
            assert_eq!(parse(&hex(*rgb)), Some(*rgb));
            assert_eq!(index_of(Some(&hex(*rgb))), i);
        }
        assert_eq!(index_of(None), 0);
        assert_eq!(index_of(Some("nonsense")), 0);
        assert_eq!(parse("12345"), None);
        assert_eq!(parse("GGGGGG"), None);
    }
}
